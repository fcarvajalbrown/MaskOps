# LinkedIn: A native Polars plugin in Rust

- Date: 2026-07-23
- Pairs with dev.to: https://dev.to/fcarvajalbrown/writing-a-native-polars-plugin-in-rust-pii-masking-with-no-per-row-python-2co8
- Cover: tools/social/linkedin/polars-plugin-rust-linkedin.png (1200x627)
- Hashtags: #Rust #Polars #DataEngineering #PrivacyEngineering
- Link placement: FIRST COMMENT, never the body (LinkedIn suppresses reach on body links)

---

## Español (principal)

Una columna de notas de pago me enseñó dónde se le va el tiempo a Python.

Texto libre: "cobrada la tarjeta 4111 1111 1111 1111", "cliente RUT 12.345.678-5 activo". Había que tapar los números y dejar la frase entera. En Python puro la forma es la de siempre: una función que se llama fila por fila. Con mil filas no lo notas. Con un millón, el intérprete es el trabajo.

Polars corre el escaneo en Rust y después te devuelve cada valor a Python para mirarlo solo. El motor vectorizado se queda quieto abajo.

¿La salida? Escribir la parte cara una sola vez, en Rust, y registrarla como una expresión de Polars. La función recibe la columna entera como un Series sobre buffers de Arrow y devuelve un Series. Python se llama una vez, no una vez por fila.

Escribí el paso a paso completo: la macro polars_expr, cómo un chequeo de bytes barato descarta la mayoría de las filas antes de correr un solo regex, y por qué is_elementwise es una promesa de la que el optimizador se cuelga, no una pista. El ejemplo real es MaskOps, la herramienta de enmascaramiento de PII que construyo, gratis y open source.

No es magia. Es cruzar la frontera entre lenguajes una sola vez por columna, en código compilado. El impuesto por fila deja de pagarse.

#Rust #Polars #DataEngineering #PrivacyEngineering

### Primer comentario (ES)

El artículo completo, gratis y sin registro: https://dev.to/fcarvajalbrown/writing-a-native-polars-plugin-in-rust-pii-masking-with-no-per-row-python-2co8

MaskOps en GitHub: https://github.com/fcarvajalbrown/MaskOps

---

## English

A column of payment notes taught me where Python spends its time.

Free text: "charged card 4111 1111 1111 1111", "cliente RUT 12.345.678-5 activo". Redact the numbers, keep the sentence. In pure Python the shape is the usual one: a function called once per row. At a thousand rows you never notice. At a million, the interpreter is the workload.

Polars runs the scan in Rust, then hands every value back to Python to look at alone. The vectorized engine underneath sits idle.

The way out? Write the expensive part once, in Rust, and register it as a Polars expression. The function receives the whole column as a Series over Arrow buffers and returns a Series. Python is called once, not once per row.

I wrote the full walkthrough: the polars_expr macro, how a cheap byte check rejects most rows before a single regex runs, and why is_elementwise is a promise the query optimizer leans on rather than a hint. The worked example is MaskOps, the PII masking tool I build, free and open source.

It is not magic. It is crossing the language boundary once per column, in compiled code. The per-row Python tax stops being paid.

#Rust #Polars #DataEngineering #PrivacyEngineering

### First comment (EN)

The full article, free and no signup: https://dev.to/fcarvajalbrown/writing-a-native-polars-plugin-in-rust-pii-masking-with-no-per-row-python-2co8

MaskOps on GitHub: https://github.com/fcarvajalbrown/MaskOps
