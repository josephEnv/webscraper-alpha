# 🤖 Automatización del SRI 🇪🇨

¡Bienvenido al proyecto de automatización del Servicio de Rentas Internas (SRI) de Ecuador! Este script automatiza diversas tareas relacionadas con la interacción y gestión de documentos en el portal en línea del SRI.

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

## 🤝 Contribuir

¡Puedes considerar contribuir al proyecto! Hay varias formas en las que puedes participar y mejorar este script:

1. **Reportar problemas**: Si encuentras algún error o problema, por favor crea un issue en el repositorio para que podamos abordarlo.
2. **Solicitar características**: Si tienes ideas para nuevas características o mejoras, siéntete libre de abrir un issue y compartir tus sugerencias.
3. **Enviar pull requests**: Si deseas contribuir directamente al código, puedes hacerlo enviando un pull request. Asegúrate de seguir las pautas de contribución y explicar claramente los cambios propuestos.

Para contribuir al proyecto, sigue estos pasos:

1. Haz un fork del repositorio (si aún no lo has hecho).
2. Clona tu fork en tu máquina local.
3. Crea una rama para tu contribución (`git checkout -b feature/nueva-caracteristica`).
4. Realiza tus cambios y asegúrate de que todo funcione correctamente.
5. Haz commit de tus cambios (`git commit -am 'Añadir nueva característica'`).
6. Sube tus cambios a tu repositorio en GitHub (`git push origin feature/nueva-caracteristica`).
7. Abre un pull request en el repositorio original y describe tus cambios detalladamente.

¡Esperamos tu contribución! 🎉



