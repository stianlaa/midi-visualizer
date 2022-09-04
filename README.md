# midi-visualizer
A visualization repo for displaying notes played on the [Circle of fifths](https://en.wikipedia.org/wiki/Circle_of_fifths). 

## About the Circle of fifths
The circle of fifths shows the order of notes you get if you jump five keys at a time on the keyboard 12 times. It can be used to show symmetry and harmony in note transitions which are harder to spot in other visualizations.

![Usage example of midi visualizer](midi_visualizer_example.gif?raw=true "A brief example of the visualizer in use")

The first three note transitions sound harmonic and nice, they are referred to as "perfect fifths", and are visualized as one step from each other. The fourth transition sounds very discordant and wrong, and is visualized as a jump straight accross the circle. Then the the first two bars of House of the rising by The Animals is played.

## Structure

The visualizer consists of two parts, a simple svelte single-page-application and a rust backend which listens to midi signals from a keyboard, and publishes this information on a websocket.

## Getting started

The rust and svelte apps have their own readme, showing how the visualizer can be used.