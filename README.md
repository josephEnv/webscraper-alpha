# 🤖 Automatización del SRI 🇪🇨

¡Bienvenido al proyecto de automatización del Servicio de Rentas Internas (SRI) de Ecuador! Este script automatiza diversas tareas relacionadas con la interacción y gestión de documentos en el portal en línea del SRI.

[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/C0C4V171M)

## Funcionalidades 🛠️

- **Inicio de sesión**: Accede al portal del SRI utilizando credenciales proporcionadas.
- **Descarga de documentos**: Automatiza la descarga de facturas, notas de crédito, notas de débito y comprobantes de retención.
- **Resolución de CAPTCHA**: Sortea los CAPTCHA que se presentan durante la interacción con el portal.
- **Transcripción de audio a texto**: Convierte archivos de audio descargados en transcripciones de texto.

## Requisitos 📋

- Instalación de Rust 🦀
- Configuración de Chromedriver 🚗
- API Key de OpenAI para la transcripción de audio 🔑

## Uso 🚀

1. Clona este repositorio.
2. Instala las dependencias especificadas en el archivo `Cargo.toml`.
3. Ejecuta el script con `cargo run --release`.

Asegúrate de proporcionar las credenciales correctas y la API Key de OpenAI para el funcionamiento óptimo del script.

## Notas importantes ℹ️

- Este script ha sido diseñado para un entorno específico y puede necesitar ajustes para adaptarse a cambios en el portal del SRI.
- La resolución de CAPTCHA puede ser sensible a cambios en la estructura del sitio web y puede requerir actualizaciones.

¡Disfruta de la automatización con el SRI y simplifica tus tareas!

**¡Atención!** Es **prohibido** comercializar con este código o seras sometido a procesos legales

## Obtener Ayuda 🆘

Para obtener ayuda sobre cómo utilizar el script, utiliza el argumento `--help` al ejecutarlo.

```bash
cargo run -- --help
```

## 💡 ¿Cómo colaborar?

¡Gracias por tu interés en contribuir al proyecto! Hay varias formas en las que puedes ayudar:

### 1. Reportar problemas

Si encuentras algún problema o tienes una sugerencia para mejorar el proyecto, por favor [abre una issue](https://github.com/josephEnv/webscraper-alpha/issues) en GitHub. Asegúrate de proporcionar toda la información relevante y una descripción clara del problema.

### 2. Contribuir con código

Si quieres contribuir con código al proyecto, sigue estos pasos:

1. **Fork** el repositorio desde [aquí](https://github.com/josephEnv/webscraper-alpha/fork).
2. Clona tu fork a tu máquina local: `git clone https://github.com/TU_NOMBRE_DE_USUARIO/webscraper-alpha.git`
3. Crea una nueva rama para tu contribución: `git checkout -b nueva-funcionalidad`
4. Realiza tus cambios y realiza commits descriptivos: `git commit -am 'Añade una nueva funcionalidad'`
5. Sube tus cambios a tu fork: `git push origin nueva-funcionalidad`
6. Crea un nuevo **Pull Request** desde tu rama a la rama principal del repositorio.

### 3. Mejorar la documentación

Si encuentras que la documentación del proyecto puede ser mejorada, ¡no dudes en enviar un pull request con tus mejoras! 

### 4. Compartir el proyecto

Si encuentras útil este proyecto, ¡no dudes en compartirlo con otros! Cuantas más personas lo utilicen y contribuyan, mejor será.

¡Gracias por tu ayuda en hacer crecer este proyecto! 🌟

