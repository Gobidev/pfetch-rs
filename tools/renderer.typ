#import "@preview/ansi-render:0.6.1": *

#set page(width: auto, height: auto, margin: .5cm)

#ansi-render(
  read("./tmp"),
  theme: terminal-themes.putty,
  bold-is-bright: true,
  font: "JetBrains Mono",
  inset: 8pt,
  radius: 5pt,
)
