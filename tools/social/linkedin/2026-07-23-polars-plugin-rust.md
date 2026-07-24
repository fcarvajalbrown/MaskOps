# LinkedIn: A native Polars plugin in Rust

- Date: 2026-07-23
- Pairs with dev.to: https://dev.to/fcarvajalbrown/writing-a-native-polars-plugin-in-rust-pii-masking-with-no-per-row-python-2co8
- Cover: tools/social/linkedin/polars-plugin-rust-linkedin.png (1200x627)
- Hashtags: #Rust #Polars #DataEngineering
- Link placement: FIRST COMMENT, never the body (LinkedIn suppresses reach on body links)
- Note: short/light Friday version (a longer version is in git history if needed)

---

## Español (principal)

Enmascarar PII fila por fila en Python funciona bien. Hasta que llegas al millón de filas y el intérprete se vuelve el cuello de botella.

La salida: escribir la parte cara una sola vez, en Rust, y registrarla como una expresión de Polars. Una llamada por columna, no una por fila.

Dejé el paso a paso escrito: la macro polars_expr, el chequeo de bytes que se salta el regex en las filas limpias, y por qué is_elementwise importa. El ejemplo es MaskOps, mi herramienta de PII, gratis y open source.

No es magia. Es cruzar la frontera entre lenguajes una sola vez.

#Rust #Polars #DataEngineering

### Primer comentario (ES)

El artículo completo, gratis y sin registro: https://dev.to/fcarvajalbrown/writing-a-native-polars-plugin-in-rust-pii-masking-with-no-per-row-python-2co8

MaskOps en GitHub: https://github.com/fcarvajalbrown/MaskOps

---

## English

Masking PII row by row in Python works fine. Until you hit a million rows and the interpreter becomes the bottleneck.

The fix: write the expensive part once, in Rust, and register it as a Polars expression. One call per column, not one per row.

I wrote up the walkthrough: the polars_expr macro, the byte check that skips the regex on clean rows, and why is_elementwise matters. The example is MaskOps, my PII tool, free and open source.

It is not magic. It is crossing the language boundary once.

#Rust #Polars #DataEngineering

### First comment (EN)

The full article, free and no signup: https://dev.to/fcarvajalbrown/writing-a-native-polars-plugin-in-rust-pii-masking-with-no-per-row-python-2co8

MaskOps on GitHub: https://github.com/fcarvajalbrown/MaskOps
