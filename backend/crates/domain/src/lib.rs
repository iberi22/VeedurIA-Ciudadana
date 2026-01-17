use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContratoSecop {
    #[serde(default)]
    pub id_contrato: Option<String>,
    #[serde(default)]
    pub nombre_entidad: Option<String>,
    #[serde(default)]
    pub nit_entidad: Option<String>,
    #[serde(default)]
    pub departamento: Option<String>,
    #[serde(default)]
    pub ciudad: Option<String>,
    #[serde(default)]
    pub objeto_del_contrato: Option<String>,
    #[serde(default)]
    pub tipo_de_contrato: Option<String>,
    #[serde(default)]
    pub modalidad_de_contratacion: Option<String>,
    #[serde(default)]
    pub valor_del_contrato: Option<String>,
    #[serde(default)]
    pub nombre_contratista: Option<String>,
    #[serde(default)]
    pub nit_contratista: Option<String>,
    #[serde(default)]
    pub fecha_de_firma: Option<String>,
    #[serde(default)]
    pub fecha_de_inicio_del_contrato: Option<String>,
    #[serde(default)]
    pub duracion: Option<String>,
}
