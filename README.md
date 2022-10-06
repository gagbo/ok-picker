<!--
SPDX-FileCopyrightText: 2022 Gerry Agbobada <git@gagbo.net>

SPDX-License-Identifier: CC0-1.0
-->

![OkHSV colour picker widget](https://i.imgur.com/MNIiV9H.png)
![OkHSL colour picker widget](https://i.imgur.com/eGTA5G2.png)

# Ok Picker

A toy application to test egui programming.

This application will be a color picker based on the
[Okhsv](https://bottosson.github.io/posts/colorpicker) colorspace by Björn
Ottosson, with:

- A 2D picker in the SV space,
- A 1D picker in the Hue space,
- Buttons to increment/decrement any single value of the HSV triple,

And as stretch goals:
- Buttons to add/remove colors to a palette,
- Contrast between each color of the palette and a chosen background color,
- A palette generator that auto-picks colours starting from 1 or 2 colors and a
  set of rules:
  + Just the foreground color
  + A foreground and a background color
  + A foreground and an accent color
