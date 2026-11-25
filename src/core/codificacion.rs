#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Codificacion {
    NrzL,
    NrzI,
    BipolarAmi,
    Pseudoternaria,
    Manchester,
    ManchesterDiferencial,
    B8zs,
}

impl Codificacion {
    pub const TODAS: [Codificacion; 7] = [
        Codificacion::NrzL,
        Codificacion::NrzI,
        Codificacion::BipolarAmi,
        Codificacion::Pseudoternaria,
        Codificacion::Manchester,
        Codificacion::ManchesterDiferencial,
        Codificacion::B8zs,
    ];

    pub fn nombre(self) -> &'static str {
        match self {
            Codificacion::NrzL => "NRZ-L",
            Codificacion::NrzI => "NRZ-I",
            Codificacion::BipolarAmi => "Bipolar AMI",
            Codificacion::Pseudoternaria => "Pseudoternaria",
            Codificacion::Manchester => "Manchester",
            Codificacion::ManchesterDiferencial => "Manchester Diferencial",
            Codificacion::B8zs => "B8ZS",
        }
    }

    pub fn descripcion(self) -> &'static str {
        match self {
            Codificacion::NrzL => "0 = nivel alto, 1 = nivel bajo. El nivel se mantiene todo el bit.",
            Codificacion::NrzI => {
                "1 = transición al inicio del bit, 0 = sin transición. (Nivel previo: bajo)"
            }
            Codificacion::BipolarAmi => {
                "0 = sin señal, 1 = pulso que alterna entre +V y -V. (Primer 1: +V)"
            }
            Codificacion::Pseudoternaria => {
                "1 = sin señal, 0 = pulso que alterna entre +V y -V. (Primer 0: +V)"
            }
            Codificacion::Manchester => {
                "Transición a mitad del bit: 0 = de alto a bajo, 1 = de bajo a alto."
            }
            Codificacion::ManchesterDiferencial => {
                "Siempre hay transición a mitad del bit. 0 = transición al inicio, 1 = sin transición al inicio. (Nivel previo: alto)"
            }
            Codificacion::B8zs => {
                "AMI en el que 8 ceros seguidos se sustituyen por 000VB0VB (V = violación, B = pulso bipolar válido)."
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Senal {
    pub niveles: Vec<[i8; 2]>,
    pub marcas: Vec<Option<&'static str>>,
}

pub fn leer_bits(texto: &str) -> Result<Vec<bool>, char> {
    texto
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| match c {
            '0' => Ok(false),
            '1' => Ok(true),
            otro => Err(otro),
        })
        .collect()
}

pub fn codificar(cod: Codificacion, bits: &[bool]) -> Senal {
    let mut marcas = vec![None; bits.len()];
    let niveles = match cod {
        Codificacion::NrzL => bits.iter().map(|&b| plano(if b { -1 } else { 1 })).collect(),
        Codificacion::NrzI => {
            let mut nivel = -1;
            bits.iter()
                .map(|&b| {
                    if b {
                        nivel = -nivel;
                    }
                    plano(nivel)
                })
                .collect()
        }
        Codificacion::BipolarAmi => alternante(bits, true),
        Codificacion::Pseudoternaria => alternante(bits, false),
        Codificacion::Manchester => bits.iter().map(|&b| if b { [-1, 1] } else { [1, -1] }).collect(),
        Codificacion::ManchesterDiferencial => {
            let mut nivel = 1;
            bits.iter()
                .map(|&b| {
                    let primera = if b { nivel } else { -nivel };
                    nivel = -primera;
                    [primera, nivel]
                })
                .collect()
        }
        Codificacion::B8zs => b8zs(bits, &mut marcas),
    };
    Senal { niveles, marcas }
}

fn plano(nivel: i8) -> [i8; 2] {
    [nivel, nivel]
}

fn alternante(bits: &[bool], pulso: bool) -> Vec<[i8; 2]> {
    let mut ultimo = -1;
    bits.iter()
        .map(|&b| {
            if b == pulso {
                ultimo = -ultimo;
                plano(ultimo)
            } else {
                plano(0)
            }
        })
        .collect()
}

fn b8zs(bits: &[bool], marcas: &mut [Option<&'static str>]) -> Vec<[i8; 2]> {
    let mut niveles = Vec::with_capacity(bits.len());
    let mut ultimo: i8 = -1;
    let mut i = 0;
    while i < bits.len() {
        if bits[i] {
            ultimo = -ultimo;
            niveles.push(plano(ultimo));
            i += 1;
        } else if bits.len() - i >= 8 && bits[i..i + 8].iter().all(|&b| !b) {
            // 000VB0VB: V repite la polaridad del pulso anterior (violación),
            // B la alterna. La polaridad final queda igual a la inicial.
            let v = ultimo;
            let patron = [0, 0, 0, v, -v, 0, -v, v];
            let nombres = [None, None, None, Some("V"), Some("B"), None, Some("V"), Some("B")];
            for k in 0..8 {
                niveles.push(plano(patron[k]));
                marcas[i + k] = nombres[k];
            }
            i += 8;
        } else {
            niveles.push(plano(0));
            i += 1;
        }
    }
    niveles
}

#[cfg(test)]
mod tests {
    use super::*;

    fn onda(cod: Codificacion, texto: &str) -> String {
        let bits = leer_bits(texto).unwrap();
        codificar(cod, &bits)
            .niveles
            .iter()
            .flatten()
            .map(|n| match n {
                1 => '+',
                -1 => '-',
                _ => '0',
            })
            .collect()
    }

    fn por_bit(cod: Codificacion, texto: &str) -> String {
        onda(cod, texto).chars().step_by(2).collect()
    }

    #[test]
    fn lee_bits() {
        assert_eq!(leer_bits("10 1"), Ok(vec![true, false, true]));
        assert_eq!(leer_bits("102"), Err('2'));
        assert_eq!(leer_bits(""), Ok(vec![]));
    }

    #[test]
    fn nrz_l() {
        assert_eq!(por_bit(Codificacion::NrzL, "01001100011"), "+-++--+++--");
    }

    #[test]
    fn nrz_i() {
        assert_eq!(por_bit(Codificacion::NrzI, "01001100011"), "-+++-++++-+");
    }

    #[test]
    fn ami() {
        assert_eq!(por_bit(Codificacion::BipolarAmi, "01001100011"), "0+00-+000-+");
    }

    #[test]
    fn pseudoternaria() {
        assert_eq!(por_bit(Codificacion::Pseudoternaria, "01001100011"), "+0-+00-+-00");
    }

    #[test]
    fn manchester() {
        assert_eq!(onda(Codificacion::Manchester, "0100"), "+--++-+-");
    }

    #[test]
    fn manchester_diferencial() {
        assert_eq!(onda(Codificacion::ManchesterDiferencial, "0100"), "-++-+-+-");
    }

    #[test]
    fn b8zs_tras_pulso_negativo() {
        assert_eq!(
            por_bit(Codificacion::B8zs, "1100000000110000010"),
            "+-000-+0+-+-00000+0"
        );
    }

    #[test]
    fn b8zs_tras_pulso_positivo() {
        assert_eq!(por_bit(Codificacion::B8zs, "1000000001"), "+000+-0-+-");
    }

    #[test]
    fn b8zs_marcas_y_ceros_cortos() {
        let bits = leer_bits("100000000").unwrap();
        let s = codificar(Codificacion::B8zs, &bits);
        assert_eq!(s.marcas[4], Some("V"));
        assert_eq!(s.marcas[5], Some("B"));
        assert_eq!(por_bit(Codificacion::B8zs, "10000000"), "+0000000");
    }
}
