Welcome to dingus engine!

Built to be highly customizeable, modular and flexible to any use case. 
Contrary to its name, it needs more technical knowledge than other game engines, and is recommended for experienced rust users. (my code is just messy lmao)

Anyways, the big gimmick is the instance system inspired by Roblox Studio, which allows game objects to be integrated and used with the game engine easier.
The engine comes with lots of built in traits, like `Renderable`, that let you define how your custom instances interact with the game engine. 
For example you could implement the `render(&self, renderer: &mut Renderer)` from `Renderable` to draw a ui object from its struct fields, and it will show up in the game engine.

The goal of Dingus Engine is for you to mold it into what you need for your project easily.

Plans for the future: 
- Right now it's in a framework stage, and I plan to add an engine application
- A hierarchy system and a set of built in services with the game engine when that happens.
- integrate with an embedded "scripting" language close to rust made by @unsigned_float_integer

Even once the engine is implemented, I still plan to add ways to interact with the engines guts. (basically open source but you don't have to dig through my messy ass code )


I'm too lazy to document what I have right now, but uhh come check back soon!
