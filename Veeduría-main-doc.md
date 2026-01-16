# **Informe Técnico Integral: Viabilidad, Arquitectura e Implementación de un Sistema Nacional de Veeduría Ciudadana Basado en Datos Abiertos, Inteligencia Artificial y Rust**

## **1\. Resumen Ejecutivo**

El presente documento técnico constituye un análisis exhaustivo sobre la viabilidad, diseño arquitectónico y estrategia de implementación para un sistema de veeduría ciudadana digital en Colombia. Este sistema tiene como objetivo central democratizar el control fiscal mediante la sincronización automatizada de datos públicos, el procesamiento analítico de alto rendimiento y la asistencia de Inteligencia Artificial (IA) para la toma de decisiones. La propuesta evaluada plantea un cambio de paradigma: transitar de la veeduría manual y reactiva a una **veeduría algorítmica, preventiva y sistemática**, soportada íntegramente sobre infraestructura de código abierto (Open Source) y metodologías de "Infraestructura como Código" y "Datos como Código".

El análisis confirma que la arquitectura propuesta, basada en el uso de **GitHub** como plataforma de orquestación y almacenamiento, y **Rust** como lenguaje de programación para el procesamiento analítico, es **técnicamente viable**, aunque enfrenta desafíos significativos en términos de escalabilidad de almacenamiento y límites de cuota en las capas gratuitas de los servicios en la nube. La elección de Rust se revela como estratégica y crítica; su eficiencia en la gestión de memoria y su capacidad para generar binarios estáticos de alto rendimiento permiten ejecutar cargas de trabajo de Extracción, Transformación y Carga (ETL) y modelos de Machine Learning (ML) dentro de los contenedores efímeros y limitados de **GitHub Actions**, una tarea que sería inviable o costosa con lenguajes interpretados como Python en el mismo entorno.

El informe detalla la riqueza del ecosistema de datos abiertos de Colombia, liderado por la plataforma datos.gov.co (basada en tecnología Socrata) y el Sistema Electrónico de Contratación Pública (SECOP II). Se identifican los mecanismos precisos de ingestión de datos a través de la API SODA, las estrategias de paginación y autenticación necesarias para evitar bloqueos por límites de tasa, y la estructura de los datasets críticos para la detección de fraude. Asimismo, se profundiza en la aplicación de algoritmos de detección de anomalías, desde la Ley de Benford hasta modelos de lenguaje (Transformers) cuantizados ejecutados localmente mediante el framework **Candle**, permitiendo análisis semántico de los objetos contractuales sin dependencias externas costosas. Finalmente, se propone una arquitectura de almacenamiento híbrido que integra **Hugging Face Hub** para gestionar el volumen masivo de datos históricos, superando las restricciones de almacenamiento de GitHub, y garantizando así la sostenibilidad del proyecto a largo plazo.

## ---

**2\. El Paradigma de la Veeduría Digital en el Contexto Colombiano**

### **2.1. Evolución del Gobierno Digital y la Transparencia**

La transformación digital del Estado colombiano no ha sido un evento aislado, sino un proceso evolutivo respaldado por un marco jurídico robusto que favorece la creación de herramientas de control ciudadano. La promulgación de la **Ley 1712 de 2014**, conocida como la Ley de Transparencia y del Derecho de Acceso a la Información Pública Nacional, marcó un hito fundamental al cambiar la lógica del secreto por el principio de "máxima publicidad" y "divulgación proactiva".1 Este marco legal obliga a las entidades estatales no solo a responder peticiones, sino a publicar activamente sus datos en formatos abiertos, estandarizados e interoperables.

La política de Gobierno Digital, impulsada por el Ministerio de Tecnologías de la Información y las Comunicaciones (MinTIC), ha establecido directrices claras mediante la **Resolución 1519 de 2020**, que estandariza la publicación de información en sedes electrónicas, y más recientemente, el **CONPES 4144 de 2024**, que formula la Política Nacional de Inteligencia Artificial. Este último documento es crucial para el proyecto propuesto, ya que legitima y promueve el uso de sistemas inteligentes para la generación de valor público y la transparencia administrativa.2 En este contexto, un sistema de veeduría asistido por IA no es una anomalía, sino una herramienta alineada con la visión estratégica del Estado, lo que sugiere una baja fricción normativa para el acceso a los datos, siempre que se respeten los protocolos de privacidad.

### **2.2. La Necesidad de la Automatización en el Control Fiscal**

El volumen de la contratación pública en Colombia es inabarcable para la auditoría humana tradicional. Con millones de contratos celebrados anualmente a través de plataformas como SECOP I y SECOP II, la capacidad de los organismos de control (Contraloría, Procuraduría) y de las veedurías ciudadanas se ve desbordada. La veeduría tradicional opera por muestreo o por denuncia reactiva; el sistema propuesto permite una **veeduría censal**, donde el 100% de los contratos son analizados automáticamente en busca de patrones atípicos.

La integración de tecnologías de análisis masivo de datos permite pasar de la sospecha anecdótica a la evidencia estadística. Al sincronizar datos de todo el país en un repositorio unificado, es posible detectar redes de corrupción que operan transversalmente en múltiples municipios, identificando patrones de contratación que serían invisibles si se audita cada entidad por separado. El uso de **GitHub** como plataforma de alojamiento no solo reduce costos, sino que garantiza la **trazabilidad inmutable** de la información: si un registro es alterado en la fuente oficial después de haber sido capturado, el historial de versiones del repositorio (Git) conservará la evidencia del cambio, proporcionando una capa de seguridad y auditoría forense sobre los propios datos públicos.

## ---

**3\. Análisis Técnico del Ecosistema de Datos Abiertos**

La viabilidad del sistema depende intrínsecamente de la calidad, disponibilidad y accesibilidad técnica de las fuentes de datos gubernamentales. Colombia dispone de una infraestructura centralizada en datos.gov.co, operada sobre la tecnología **Socrata**, lo cual presenta ventajas y desafíos específicos para la ingestión automatizada.

### **3.1. Infraestructura Socrata y la API SODA**

A diferencia de otras plataformas de datos abiertos como CKAN (utilizada predominantemente en Europa), Socrata es una solución SaaS (Software as a Service) que ofrece capacidades avanzadas de consulta a través de su API, conocida como **SODA (Socrata Open Data API)**.3 Esta distinción es crítica para la arquitectura del sistema propuesto. En un portal basado en archivos estáticos, el sistema de veeduría tendría que descargar gigabytes de archivos CSV completos cada día para procesarlos, lo cual saturaría rápidamente el ancho de banda y el almacenamiento.

Socrata, en cambio, permite realizar consultas granulares directamente sobre el endpoint HTTP utilizando **SoQL (Socrata Query Language)**, un lenguaje similar a SQL. Esto permite trasladar gran parte de la carga de filtrado y agregación al servidor del gobierno antes de la transferencia de datos. Por ejemplo, en lugar de descargar todos los contratos, el sistema puede solicitar únicamente aquellos firmados en las últimas 24 horas, o aquellos cuyo valor supere cierto umbral, o filtrar por departamentos específicos.4

Mecanismos de Autenticación y Límites de Tasa (Rate Limiting):  
El acceso a la API de Socrata puede realizarse de manera anónima, pero esto expone al sistema a límites de tasa restrictivos basados en la dirección IP. Dado que los runners de GitHub Actions comparten rangos de direcciones IP con miles de usuarios a nivel mundial, es altamente probable que las peticiones anónimas sean bloqueadas o limitadas (throttled) agresivamente. Para mitigar este riesgo, es imperativo el uso de App Tokens. Al registrar la aplicación en el portal de desarrolladores de Socrata y enviar el token en el encabezado X-App-Token de cada petición, el sistema accede a un pool de cuotas exclusivo, permitiendo hasta 1,000 peticiones por hora rodante, lo cual es suficiente para una estrategia de sincronización incremental bien diseñada.5

### **3.2. Fuentes de Datos Primarias: SECOP II**

El Sistema Electrónico de Contratación Pública (SECOP II) es la fuente de verdad transaccional y el núcleo del análisis de veeduría. A diferencia de su predecesor (SECOP I), SECOP II es transaccional, lo que implica una estructura de datos más compleja y rica.

* **Estructura del Dataset:** El conjunto de datos *SECOP II \- Contratos Electrónicos* (ID: jbjy-vk9h) es masivo, conteniendo millones de registros históricos.6 Cada fila representa un contrato y contiene columnas críticas para el análisis de fraude:  
  * valor\_del\_contrato: Fundamental para análisis de Benford y detección de sobrecostos.  
  * nombre\_contratista y nit\_contratista: Permiten la identificación única de proveedores y el rastreo de su historial en múltiples entidades.  
  * objeto\_del\_contrato: Texto libre que describe el servicio o bien. Es el insumo principal para los modelos de Procesamiento de Lenguaje Natural (NLP).  
  * modalidad\_de\_contratacion: Permite filtrar entre licitaciones públicas, contratación directa, subastas inversas, etc.  
  * fecha\_de\_firma y duracion: Esenciales para análisis de series de tiempo y detección de contratos "express".

Desafíos de Calidad de Datos:  
A pesar de la estandarización, los datos presentan inconsistencias. Los campos de texto libre suelen contener errores ortográficos, variaciones en la escritura de nombres (e.g., "Alcaldía de Bogotá" vs. "Bogotá D.C., Alcaldía") y datos faltantes. El sistema de veeduría debe incluir una etapa robusta de limpieza y normalización de datos (Data Wrangling) utilizando Rust antes de cualquier análisis estadístico.8

### **3.3. Fuentes Complementarias: SIGEP y Listas de Sanciones**

Para cerrar el ciclo de veeduría, es necesario cruzar la información contractual con los datos de los funcionarios públicos. El **SIGEP (Sistema de Información y Gestión del Empleo Público)** provee datasets sobre las hojas de vida y declaraciones de bienes y rentas de los servidores públicos.9

* **Detección de Conflictos de Interés:** Al cruzar los NITs de los contratistas ganadores en SECOP II con las declaraciones de bienes y rentas y los registros de parentesco en SIGEP, el sistema puede identificar posibles conflictos de interés no declarados.  
* **Consideraciones de Privacidad:** El acceso y republicación de estos datos debe ser extremadamente cuidadoso para no violar la Ley de Habeas Data. Aunque la información es pública por ley de transparencia, su agregación y perfilamiento masivo puede tener implicaciones legales. La arquitectura debe contemplar la anonimización o pseudonimización de identificadores personales (cédulas) en los reportes públicos generados.

## ---

**4\. Arquitectura Tecnológica: Rust y el Patrón "Flat Data"**

La arquitectura propuesta por el usuario se alinea con el patrón de diseño conocido como **"Flat Data"** o "Git Scraping", una técnica innovadora que utiliza sistemas de control de versiones para la gestión de datos vivos.

### **4.1. El Patrón "Flat Data" en GitHub**

La premisa central es utilizar el repositorio de GitHub como la base de datos maestra. Un flujo de trabajo automatizado (GitHub Actions) se ejecuta periódicamente, obtiene los datos frescos, los procesa y realiza un "commit" de los cambios al repositorio.11

**Ventajas Estratégicas:**

1. **Historial Inmutable:** Git proporciona un historial de cambios auditables ("audit log") perfecto. Si un contrato es modificado retroactivamente en SECOP II para ocultar irregularidades, el sistema de veeduría conservará la versión original y registrará el "diff" (diferencia) exacto, evidenciando la alteración.  
2. **Infraestructura Serverless Gratuita:** Al utilizar los *runners* de GitHub Actions, se elimina la necesidad de mantener servidores VPS o instancias EC2 costosas. Los repositorios públicos gozan de minutos de ejecución gratuitos generosos, lo que hace viable el proyecto desde el punto de vista financiero para la sociedad civil.13  
3. **Transparencia del Algoritmo:** Al ser Open Source, el código que detecta la corrupción es público. Esto protege a la veeduría de acusaciones de sesgo político; cualquier ciudadano puede auditar el código para verificar que los criterios de alerta son matemáticos y objetivos.

### **4.2. Rust: El Motor de Eficiencia Necesario**

La elección de **Rust** sobre lenguajes más tradicionales en ciencia de datos como Python es la decisión técnica más crítica y acertada para este entorno específico. Los *runners* estándar de GitHub Actions (ubuntu-latest) tienen recursos limitados: 2 vCPUs y 7 GB de memoria RAM.15

**Comparativa de Rendimiento y Memoria:**

* **Gestión de Memoria:** Cargar un dataset de varios millones de filas (como SECOP II) en Python con Pandas implica un consumo de memoria masivo debido a la sobrecarga de objetos de Python (boxing). Frecuentemente, esto resulta en errores de *Out Of Memory (OOM)* y la terminación abrupta del proceso. Rust, con su modelo de propiedad (ownership) y sin recolector de basura (Garbage Collector), permite un control preciso de la memoria. Librerías como **Polars** en Rust están diseñadas para operar "Out-of-Core" o mediante *streaming*, procesando datasets más grandes que la RAM disponible sin colapsar.16  
* **Velocidad de Ejecución:** GitHub Actions factura (o descuenta cuota) por minuto. Rust, al ser un lenguaje compilado a código máquina nativo, ejecuta tareas de ETL y algoritmos matemáticos órdenes de magnitud más rápido que Python interpretado. Esto permite realizar análisis más complejos (como correr modelos de IA) dentro de la ventana de tiempo permitida por GitHub (máximo 6 horas, aunque se recomienda mantenerlo bajo minutos).18  
* **Despliegue Simplificado:** Un proyecto en Rust compila en un único binario estático. Esto elimina la fragilidad de los entornos de Python (dependencias rotas, conflictos de versiones de pip/conda) y acelera el tiempo de arranque del contenedor.

### **4.3. Limitaciones de la Arquitectura y Estrategias de Mitigación**

A pesar de sus ventajas, GitHub no es una base de datos. Existen límites duros que deben ser gestionados:

* **Límite de Tamaño de Archivo:** GitHub bloquea archivos individuales mayores a 100 MB.19  
* **Límite de Tamaño del Repositorio:** Se recomienda mantener el repositorio por debajo de 1 GB para asegurar rendimiento, aunque el límite técnico es mayor (5-10 GB).20  
* **Ancho de Banda Git LFS:** Si se usa *Git Large File Storage* (LFS) para almacenar los datasets binarios, existe una cuota mensual de ancho de banda (1 GB en cuentas gratuitas) que se consumirá rápidamente si muchos usuarios clonan el repositorio.21

**Estrategia de Mitigación:** La arquitectura no debe almacenar el dataset histórico completo de SECOP en el repositorio de GitHub. Se debe adoptar un enfoque híbrido donde GitHub almacena el código y los reportes ligeros (JSON/CSV pequeños), mientras que los datos "crudos" y procesados masivos se alojan en **Hugging Face Hub** o un almacenamiento de objetos S3 compatible, gestionados desde el binario de Rust.23

## ---

**5\. Ingeniería de Datos y Algoritmos de Veeduría**

El núcleo funcional del sistema es el procesamiento de datos. Se requiere una cadena de suministro de datos (pipeline) robusta que transforme la data cruda de Socrata en insumos para la detección de fraude.

### **5.1. Estrategia de Extracción Incremental (Delta Load)**

Descargar el dataset completo de SECOP II (5M+ registros) diariamente es inviable e ineficiente. El binario de Rust debe implementar una lógica de sincronización incremental:

1. **Persistencia de Estado:** El sistema debe guardar un "cursor" o marca de tiempo (last\_sync\_date) de la última ejecución exitosa.  
2. **Consulta Delta:** Al iniciar, el binario consulta la API SODA solicitando solo los registros donde fecha\_de\_firma O last\_updated sea mayor que el cursor almacenado.  
   * *Query SoQL:* ?$where=last\_updated \> '2025-01-01T12:00:00'  
3. **Manejo de Paginación:** El cliente HTTP en Rust (usando la librería reqwest) debe iterar sobre las páginas de resultados. Es crucial manejar correctamente los códigos de estado HTTP, especialmente el 429 Too Many Requests, implementando una estrategia de "backoff exponencial" para reintentar la petición después de una espera, respetando los límites de la API.5

### **5.2. Procesamiento Analítico con Polars**

Para la manipulación de datos, la librería **Polars** es la elección superior en el ecosistema Rust.

* **Evaluación Perezosa (Lazy Evaluation):** Polars permite construir un plan de consulta (LazyFrame) que define todas las transformaciones (filtros, joins, agregaciones) pero no las ejecuta hasta el momento final (collect()). Esto permite al motor de Polars optimizar la consulta, aplicando filtros en la fuente y minimizando el uso de memoria RAM, vital para el entorno de GitHub Actions.16  
* **Paralelismo Automático:** Polars utiliza automáticamente todos los núcleos de CPU disponibles para paralelizar operaciones de columnas, maximizando el uso de las 2 vCPUs del runner.

### **5.3. Algoritmos de Detección de Fraude e Integración de IA**

El componente de "toma de decisiones asistida por IA" se materializa mediante la implementación de algoritmos de detección de anomalías y modelos de lenguaje.

#### **5.3.1. Detección Estadística: Ley de Benford y "Red Flags"**

Las primeras capas de detección no requieren IA compleja, sino estadística robusta.

* **Ley de Benford:** Este principio estadístico establece que en conjuntos de datos naturales (como los valores monetarios de contratos), el dígito 1 debería aparecer como primer dígito aproximadamente el 30% de las veces. Desviaciones significativas de esta distribución sugieren manipulación de datos o fraccionamiento de contratos.  
  * *Implementación:* Utilizando Rust, se puede calcular la distribución de los primeros dígitos de los contratos de una entidad específica y compararla con la distribución teórica de Benford usando pruebas de bondad de ajuste (Chi-cuadrado). La librería benf o cálculos directos sobre arrays de ndarray pueden facilitar esto.25  
* **Indicadores de Bandera Roja (Red Flags):** Implementación de las reglas definidas por *Open Contracting Partnership*:  
  * **Licitante Único:** Identificar procesos competitivos donde solo se presenta un oferente.  
  * **Tiempo Insuficiente:** Procesos donde el tiempo entre la publicación y el cierre es sospechosamente corto.  
  * **Concentración de Contratación:** Índice de Herfindahl-Hirschman (HHI) para detectar monopolios de facto en entidades territoriales.27

#### **5.3.2. Inteligencia Artificial y NLP con Candle**

Para analizar el contenido no estructurado (texto de los objetos contractuales), se propone el uso de **Candle**, el framework de ML minimalista de Hugging Face escrito en Rust.29

* **Por qué Candle y no Burn/Tch-rs:** Candle está diseñado específicamente para la inferencia eficiente y sin dependencias pesadas (como LibTorch o CUDA) que complicarían el entorno de CI/CD. Permite cargar modelos directamente desde archivos .safetensors.  
* **Modelo de Lenguaje:** Se recomienda utilizar modelos BERT pre-entrenados en español, como dccuchile/bert-base-spanish-wwm-cased o roberta-base-bne (entrenado con datos de la Biblioteca Nacional de España), que tienen un excelente desempeño en comprensión del idioma.31  
* **Estrategia de Cuantización:** Para ejecutar estos modelos en un runner con CPU limitada, es esencial usar versiones **cuantizadas** (e.g., int8 o q4\_0). La cuantización reduce la precisión numérica de los pesos del modelo (de 32-bit float a 8-bit integer) con una pérdida mínima de exactitud, pero reduciendo el uso de memoria y aumentando la velocidad de inferencia drásticamente.33  
* **Aplicaciones:**  
  * *Clasificación de Objetos:* Categorizar contratos automáticamente.  
  * *Detección de Similitud:* Identificar contratos fraccionados buscando objetos semánticamente idénticos redactados de forma ligeramente diferente para evadir filtros de texto exacto.

#### **5.3.3. Asistencia Generativa (LLMs)**

Para generar resúmenes explicativos ("Toma de decisiones asistida"), se puede integrar la API de **GitHub Models** (actualmente en beta gratuita) dentro del flujo. El binario de Rust puede enviar el JSON de un contrato sospechoso a un modelo como GPT-4o-mini o Llama-3 a través de la API, con un prompt diseñado para que el modelo explique *por qué* el contrato es anómalo en lenguaje natural, facilitando la comprensión del ciudadano no experto.34

## ---

**6\. Estrategia de Almacenamiento Híbrido y Escalabilidad**

Dado que el volumen de datos de SECOP II excede la capacidad de un repositorio de GitHub, se requiere una arquitectura de almacenamiento desacoplada.

### **6.1. Hugging Face Hub como "Data Lake"**

La plataforma Hugging Face (HF) no es solo para modelos, sino también para datasets. Ofrece alojamiento gratuito de datasets masivos (cientos de gigabytes) con control de versiones basado en Git LFS.

* **Integración Rust-HF:** La librería (crate) hf\_hub en Rust permite interactuar programáticamente con HF. El flujo de trabajo sería:  
  1. El binario en GitHub Actions descarga el último snapshot del dataset (formato Parquet) desde HF.  
  2. Realiza la carga incremental desde Socrata y actualiza el archivo Parquet localmente.  
  3. Sube el nuevo archivo Parquet actualizado a HF.23  
* **Formato Parquet:** Se debe estandarizar el uso de **Apache Parquet** en lugar de CSV o JSON para el almacenamiento de datos. Parquet es un formato columnar binario que ofrece una compresión superior (reduciendo archivos de GBs a MBs) y permite lecturas parciales eficientes, integrándose nativamente con Polars.37

### **6.2. Visualización y Consumo Público**

El portal público (frontend) no debe consultar Socrata ni procesar datos en vivo. Debe ser un sitio estático (generado con frameworks como Astro o Hugo, o incluso una simple SPA) alojado en **GitHub Pages**. Este portal consumirá archivos JSON ligeros y optimizados ("Datamarts") generados diariamente por el proceso de Rust y alojados en el repositorio de GitHub o en un CDN. Esto garantiza que el portal sea rápido, resiliente a picos de tráfico y tenga un costo de mantenimiento cero.

## ---

**7\. Análisis de Riesgos y Hoja de Ruta**

### **7.1. Riesgos Críticos y Mitigación**

* **Bloqueo de Socrata:** Riesgo de exceder límites de API. *Mitigación:* Uso estricto de App Tokens, backoff exponencial y sincronización incremental (nunca descargas totales).  
* **Falsos Positivos en Fraude:** Riesgo de dañar la reputación de entidades honestas por errores algorítmicos. *Mitigación:* Etiquetar claramente las alertas como "anomalías estadísticas" y no como "corrupción confirmada". Transparencia total del algoritmo utilizado.  
* **Privacidad (Habeas Data):** Riesgo de exponer datos sensibles del SIGEP. *Mitigación:* Hashing (anonimización irreversible) de números de cédula y datos de contacto en los datasets públicos generados.

### **7.2. Conclusión de Viabilidad**

La investigación concluye que el sistema propuesto es **técnicamente viable**, **económicamente sostenible** (gracias a la infraestructura gratuita de GitHub y Hugging Face) y **socialmente relevante**. La combinación de Rust para el cómputo eficiente y la arquitectura "Flat Data" supera las limitaciones tradicionales de los proyectos cívicos, permitiendo una veeduría potente y escalable.

La integración de IA mediante modelos cuantizados en Rust democratiza el acceso a herramientas de análisis forense que antes estaban reservadas para organismos de control con grandes presupuestos. La implementación de este sistema no solo es posible, sino que representa un modelo de referencia para la transparencia gubernamental en la era de la inteligencia artificial.

### **7.3. Hoja de Ruta Sugerida**

1. **Fase 1 (Cimiento):** Desarrollo del binario Rust para ingestión incremental de SECOP II y almacenamiento en Parquet sobre Hugging Face.  
2. **Fase 2 (Analítica):** Implementación de indicadores estadísticos (Benford, Red Flags) en Rust/Polars.  
3. **Fase 3 (IA):** Integración de Candle para análisis de objetos contractuales y detección de similitud.  
4. **Fase 4 (Visualización):** Despliegue del portal público en GitHub Pages alimentado por los reportes diarios.

## ---

**Tablas y Datos de Referencia**

### **Tabla 1: Comparativa de Rendimiento y Uso de Recursos (Contexto CI/CD)**

| Característica | Python (Pandas) | Rust (Polars) | Impacto en GitHub Actions |
| :---- | :---- | :---- | :---- |
| **Gestión de Memoria** | Alta sobrecarga (Objetos Python) | Zero-cost abstractions | Rust evita errores OOM en runners de 7GB. |
| **Paralelismo** | Limitado por GIL (Single core) | Multihilo nativo | Rust maximiza uso de las 2 vCPUs disponibles. |
| **Tiempo de Arranque** | Lento (Instalación de deps pip) | Instantáneo (Binario estático) | Rust ahorra minutos de facturación por ejecución. |
| **Tamaño de Despliegue** | Pesado (venv \+ librerías) | Ligero (\<50MB binario) | Rust reduce tiempos de descarga y red. |

### **Tabla 2: Mapeo de Fuentes de Datos y Usos en Veeduría**

| Fuente de Datos | Dataset ID (Socrata) | Campos Críticos | Aplicación en Veeduría |
| :---- | :---- | :---- | :---- |
| **SECOP II (Contratos)** | jbjy-vk9h | valor\_contrato, nit\_contratista, objeto | Detección de sobrecostos, Ley de Benford, NLP. |
| **SECOP II (Procesos)** | p6dx-8zbt | fecha\_publicacion, fecha\_cierre | Detección de plazos irreales ("Licitaciones sastre"). |
| **SIGEP (Funcionarios)** | (Varios) | nombre\_completo, declaracion\_bienes | Cruce de conflictos de interés y puertas giratorias. |
| **Sanciones (SIRI)** | (Indexado) | nit\_sancionado, tipo\_sancion | Verificación de antecedentes e inhabilidades. |

---

*Fin del Informe Técnico.*

#### **Obras citadas**

1. Datos abiertos \- Gobierno Digital MINTIC, fecha de acceso: enero 15, 2026, [https://gobiernodigital.mintic.gov.co/portal/Iniciativas/Datos-abiertos/](https://gobiernodigital.mintic.gov.co/portal/Iniciativas/Datos-abiertos/)  
2. Política de Gobierno Digital, fecha de acceso: enero 15, 2026, [https://www.igac.gov.co/sites/default/files/listadomaestro/PL-GET-03%20V2%20Gobierno%20Digital.pdf](https://www.igac.gov.co/sites/default/files/listadomaestro/PL-GET-03%20V2%20Gobierno%20Digital.pdf)  
3. CKAN vs SOCRATA \- Datos Abiertos, fecha de acceso: enero 15, 2026, [https://herramientas.datos.gov.co/sites/default/files/2020-11/Comparativo%20Plataformas%20Datos%20Abiertos%20Banco%20Mundial2015.pdf](https://herramientas.datos.gov.co/sites/default/files/2020-11/Comparativo%20Plataformas%20Datos%20Abiertos%20Banco%20Mundial2015.pdf)  
4. The LIMIT Clause \- Socrata API \- Data & Insights, fecha de acceso: enero 15, 2026, [https://dev.socrata.com/docs/queries/limit.html](https://dev.socrata.com/docs/queries/limit.html)  
5. Application Tokens | Socrata \- Data & Insights, fecha de acceso: enero 15, 2026, [https://dev.socrata.com/docs/app-tokens.html](https://dev.socrata.com/docs/app-tokens.html)  
6. SECOP II \- Contratos Electrónicos | Datos Abiertos Colombia, fecha de acceso: enero 15, 2026, [https://www.datos.gov.co/Estad-sticas-Nacionales/SECOP-II-Contratos-Electr-nicos/jbjy-vk9h](https://www.datos.gov.co/Estad-sticas-Nacionales/SECOP-II-Contratos-Electr-nicos/jbjy-vk9h)  
7. SECOP II \- Contratos Electrónicos | Socrata API Foundry \- Data & Insights, fecha de acceso: enero 15, 2026, [https://dev.socrata.com/foundry/www.datos.gov.co/jbjy-vk9h](https://dev.socrata.com/foundry/www.datos.gov.co/jbjy-vk9h)  
8. Agencia Nacional de Contratación Pública (Colombia Compra Eficiente) | OCP Data Registry, fecha de acceso: enero 15, 2026, [https://data.open-contracting.org/en/publication/61](https://data.open-contracting.org/en/publication/61)  
9. Conjunto servidores públicos | Datos Abiertos Colombia, fecha de acceso: enero 15, 2026, [https://www.datos.gov.co/Funci-n-P-blica/Conjunto-servidores-p-blicos/2jzx-383z](https://www.datos.gov.co/Funci-n-P-blica/Conjunto-servidores-p-blicos/2jzx-383z)  
10. Bienvenido a SIGEP II \- Función Pública, fecha de acceso: enero 15, 2026, [https://www1.funcionpublica.gov.co/web/sigep2](https://www1.funcionpublica.gov.co/web/sigep2)  
11. Flat Data · Actions · GitHub Marketplace, fecha de acceso: enero 15, 2026, [https://github.com/marketplace/actions/flat-data](https://github.com/marketplace/actions/flat-data)  
12. Flat Data \- GitHub Next, fecha de acceso: enero 15, 2026, [https://githubnext.com/projects/flat-data](https://githubnext.com/projects/flat-data)  
13. GitHub Actions billing \- GitHub Docs, fecha de acceso: enero 15, 2026, [https://docs.github.com/billing/managing-billing-for-github-actions/about-billing-for-github-actions](https://docs.github.com/billing/managing-billing-for-github-actions/about-billing-for-github-actions)  
14. Cloud ETL automation with Github Actions and Coiled, fecha de acceso: enero 15, 2026, [https://docs.coiled.io/blog/cloud-etl-automation-github-actions-coiled.html](https://docs.coiled.io/blog/cloud-etl-automation-github-actions-coiled.html)  
15. GitHub-hosted runners reference, fecha de acceso: enero 15, 2026, [https://docs.github.com/en/actions/reference/runners/github-hosted-runners](https://docs.github.com/en/actions/reference/runners/github-hosted-runners)  
16. Pandas vs Polars: Which Data Processor Runs Faster \- Shuttle.dev, fecha de acceso: enero 15, 2026, [https://www.shuttle.dev/blog/2025/09/24/pandas-vs-polars](https://www.shuttle.dev/blog/2025/09/24/pandas-vs-polars)  
17. Pandas vs. Polars: The 2026 Benchmark & Survival Guide, fecha de acceso: enero 15, 2026, [https://medium.com/@abayomiajiboye46111/pandas-vs-polars-the-2026-benchmark-survival-guide-e352c3ca4e5f](https://medium.com/@abayomiajiboye46111/pandas-vs-polars-the-2026-benchmark-survival-guide-e352c3ca4e5f)  
18. GitHub Actions Price Calculator \- Depot, fecha de acceso: enero 15, 2026, [https://depot.dev/github-actions-price-calculator](https://depot.dev/github-actions-price-calculator)  
19. About large files on GitHub \- GitHub Docs, fecha de acceso: enero 15, 2026, [https://docs.github.com/en/repositories/working-with-files/managing-large-files/about-large-files-on-github](https://docs.github.com/en/repositories/working-with-files/managing-large-files/about-large-files-on-github)  
20. Repository limits \- GitHub Docs, fecha de acceso: enero 15, 2026, [https://docs.github.com/en/repositories/creating-and-managing-repositories/repository-limits](https://docs.github.com/en/repositories/creating-and-managing-repositories/repository-limits)  
21. Git Large File Storage billing \- GitHub Docs, fecha de acceso: enero 15, 2026, [https://docs.github.com/billing/managing-billing-for-git-large-file-storage/about-billing-for-git-large-file-storage](https://docs.github.com/billing/managing-billing-for-git-large-file-storage/about-billing-for-git-large-file-storage)  
22. GitHub Storage Limits \- Blog \- GitProtect.io, fecha de acceso: enero 15, 2026, [https://gitprotect.io/blog/github-storage-limits/](https://gitprotect.io/blog/github-storage-limits/)  
23. Upload files to the Hub \- Hugging Face, fecha de acceso: enero 15, 2026, [https://huggingface.co/docs/huggingface\_hub/guides/upload](https://huggingface.co/docs/huggingface_hub/guides/upload)  
24. Rate limits for the REST API \- GitHub Docs, fecha de acceso: enero 15, 2026, [https://docs.github.com/en/rest/using-the-rest-api/rate-limits-for-the-rest-api](https://docs.github.com/en/rest/using-the-rest-api/rate-limits-for-the-rest-api)  
25. A Benford's law based method for fraud detection using R Library \- PMC \- NIH, fecha de acceso: enero 15, 2026, [https://pmc.ncbi.nlm.nih.gov/articles/PMC8720889/](https://pmc.ncbi.nlm.nih.gov/articles/PMC8720889/)  
26. benf \- crates.io: Rust Package Registry, fecha de acceso: enero 15, 2026, [https://crates.io/crates/benf](https://crates.io/crates/benf)  
27. RED FLAGS​ for integrity​: Giving the ​green light​ to open data solutions, fecha de acceso: enero 15, 2026, [https://www.open-contracting.org/wp-content/uploads/2016/11/OCP2016-Red-flags-for-integrityshared-1.pdf](https://www.open-contracting.org/wp-content/uploads/2016/11/OCP2016-Red-flags-for-integrityshared-1.pdf)  
28. Open Contracting Data Standard \- World Bank Documents, fecha de acceso: enero 15, 2026, [https://documents1.worldbank.org/curated/en/744551614955316901/pdf/Open-Contracting-Data-Standard.pdf](https://documents1.worldbank.org/curated/en/744551614955316901/pdf/Open-Contracting-Data-Standard.pdf)  
29. Building Sentence Transformers in Rust: A Practical Guide with Burn, ONNX Runtime, and Candle \- DEV Community, fecha de acceso: enero 15, 2026, [https://dev.to/mayu2008/building-sentence-transformers-in-rust-a-practical-guide-with-burn-onnx-runtime-and-candle-281k](https://dev.to/mayu2008/building-sentence-transformers-in-rust-a-practical-guide-with-burn-onnx-runtime-and-candle-281k)  
30. huggingface/candle: Minimalist ML framework for Rust \- GitHub, fecha de acceso: enero 15, 2026, [https://github.com/huggingface/candle](https://github.com/huggingface/candle)  
31. dccuchile/bert-base-spanish-wwm-cased \- Hugging Face, fecha de acceso: enero 15, 2026, [https://huggingface.co/dccuchile/bert-base-spanish-wwm-cased](https://huggingface.co/dccuchile/bert-base-spanish-wwm-cased)  
32. PlanTL-GOB-ES/roberta-base-bne \- Hugging Face, fecha de acceso: enero 15, 2026, [https://huggingface.co/PlanTL-GOB-ES/roberta-base-bne](https://huggingface.co/PlanTL-GOB-ES/roberta-base-bne)  
33. Candle BERT Semantic Similarity Wasm \- a Hugging Face Space by radames, fecha de acceso: enero 15, 2026, [https://huggingface.co/spaces/radames/Candle-BERT-Semantic-Similarity-Wasm](https://huggingface.co/spaces/radames/Candle-BERT-Semantic-Similarity-Wasm)  
34. Automate your project with GitHub Models in Actions, fecha de acceso: enero 15, 2026, [https://github.blog/ai-and-ml/generative-ai/automate-your-project-with-github-models-in-actions/](https://github.blog/ai-and-ml/generative-ai/automate-your-project-with-github-models-in-actions/)  
35. Using an LLM in GitHub Actions \- Anthony Shaw, fecha de acceso: enero 15, 2026, [https://tonybaloney.github.io/posts/using-llm-in-github-actions.html](https://tonybaloney.github.io/posts/using-llm-in-github-actions.html)  
36. Use Rust's Huggingface API \- Medium, fecha de acceso: enero 15, 2026, [https://medium.com/@mellomello2030/use-rusts-huggingface-api-0103461d72f3](https://medium.com/@mellomello2030/use-rusts-huggingface-api-0103461d72f3)  
37. dataset Load \- Hugging Face, fecha de acceso: enero 15, 2026, [https://huggingface.co/docs/datasets/loading](https://huggingface.co/docs/datasets/loading)