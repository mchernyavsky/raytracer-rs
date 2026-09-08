# raytracer

Ein in Rust implementierter Raytracer, basierend auf Peter Shirleys
[*Ray Tracing in One Weekend*](https://raytracing.github.io/books/RayTracingInOneWeekend.html).

Er rendert eine feste Szene – vier Kugeln auf einer großen Bodenkugel, beleuchtet von einem
Himmelsverlauf – und schreibt das Ergebnis als ASCII-PPM-Bild (P3).

## Funktionen

- Kantenglättung durch Mittelung mehrerer zufällig versetzter Abtastungen pro Pixel
- Diffuse (Lambert’sche) und metallische Materialien, mit einstellbarer Streuung der Reflexionen
- Rekursive Strahlreflexionen mit Tiefenbegrenzung sowie Gamma-2-Korrektur bei der Ausgabe
- Multithreading beim Rendern: Pixel werden mit [rayon](https://github.com/rayon-rs/rayon) parallel berechnet
- Szenen werden über das `Hit`-Trait zusammengesetzt, sodass sich Trefferlisten wie gewöhnliche Objekte verschachteln lassen

## Voraussetzungen

Die Binärdatei nutzt das instabile Feature `try_blocks`, daher wird **eine Nightly-Toolchain von
Rust benötigt**:

```bash
rustup toolchain install nightly
```

Der Bibliotheksteil des Crates lässt sich mit Stable kompilieren.

## Bauen und Ausführen

```bash
cargo +nightly build --release
cargo +nightly run --release -- image.ppm    # in eine Datei schreiben
cargo +nightly run --release > image.ppm     # oder nach stdout
```

Verwende `--release`; ein Debug-Build rendert sehr langsam. PPM-Dateien lassen sich mit den
meisten Bildbetrachtern öffnen und beispielsweise mit `convert image.ppm image.png` (ImageMagick)
konvertieren.

## Konfiguration

Über den optionalen Ausgabepfad hinaus gibt es keine Konfiguration per Kommandozeile. Auflösung und
Qualität sind Konstanten zur Kompilierzeit in `src/main.rs`:

| Konstante | Bedeutung |
| --- | --- |
| `IMAGE_WIDTH`, `IMAGE_HEIGHT` | Ausgabeauflösung in Pixeln |
| `N_SAMPLES` | Strahlen pro Pixel (Qualität der Kantenglättung) |
| `MAX_DEPTH` | Maximale Anzahl der Strahlreflexionen |

Die Szene selbst wird in `make_world` aufgebaut, die Ansicht in `make_camera`, beide in `src/main.rs`.

## Projektaufbau

| Modul | Inhalt |
| --- | --- |
| `vec3.rs` | Dreikomponentiger `f64`-Vektor für Punkte, Richtungen und lineare Farben |
| `ray.rs` | Strahl als Ursprung + Richtung |
| `camera.rs` | Bildet normalisierte Bildschirmkoordinaten auf Strahlen ab |
| `hit.rs` | `Hit`-Trait, `HitRecord` und `HitList` zum Zusammensetzen von Szenen |
| `sphere.rs` | Schnittpunktberechnung Kugel–Strahl |
| `material.rs` | `Scatter`-Trait mit `Lambertian` und `Metal` |
| `image.rs` | Pixelpuffer, paralleler Pixel-Iterator und PPM-Writer |
| `color.rs` | 8-Bit-RGB-Ausgabefarbe |

## Tests

Die Unit-Tests liegen direkt beim zugehörigen Code und laufen mit Stable:

```bash
cargo test
cargo test --lib vec3::tests    # ein einzelnes Modul
```

## Lizenz

MIT – siehe [LICENSE](LICENSE).
