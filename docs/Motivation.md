# Motivation - Why exactly do we need `Lala`

Consider three scenarios that could sound troublesome.

## Scenario #1: Configuration files.

```jsonc
{
    // tasks.json
    "version": "2.0.0",
    "tasks": [
        {
            "type": "cargo",
            "command": "clean",
            "problemMatcher": [
                "$rustc"
            ],
            "label": "lala-proj clean"
        },
        {
            "type": "cargo",
            "command": "build",
            "problemMatcher": [
                "$rustc"
            ],
            "label": "lala-proj build"
        },
        {
            "type": "cargo",
            "command": "test",
            "problemMatcher": [
                "$rustc"
            ],
            "label": "lala-proj test"
        }
    ]
}
```

The example above is a piece of a configuration file that I've just created in vs-code. You may have noticed the duplication: all tasks fall into the same pattern, yet I have no reasonable way to remove such duplication or, say, create a higher level of abstraction. Imagine how the configurations would pile up, and suddenly one day, you find yourself adding a new field to every single entry only because vs-code has updated its settings! (which has already happened to me, sadly)

Observation #1: What we traditionally defined as "data file" has limited or no consideration of human-readable abstraction.

## Scenario #2: Describing an FSM. Or simply, a chatbot.

Recently, I'm writing a personal chatbot. The process is roughly like the following:
<details open>
<summary>Click to collapse</summary>
<image src="./chatbot_graph.png" alt="Chatbot FSM graph" />
</details>

In the graph above, my idea and needs are clearly stated; but when I put them down in my code (like python), the target language may not accept them gracefully. There exist at least three possible options:

1. Use functions to capture the relation graph and write handlers over and over. It's hard to scale and, happy debugging :-)
2. Write a "scheme" file that describes the graph above and later interprets it in the target language:
   ```jsonc
   {
       // A chatbot scheme.
       // "!verb" will be interpreted as an action, like get the weather info online and send
       // "@condition" will be a boolean satisfying some state
       "weather": {
           "entrance": "r_weather",               // starts at "r_weather" (read)
           "dialog": {
               "r_weather": {
                   "interact": "!send_weather",   // tell the bot to send weather, perhaps by calling functions
                   "step": {
                       "!end": "@true"            // jumps to the state of end, always
                   }
               }
           }
       },
       "todo": {
           "entrance": "r_show",
           "dialog": {
                "r_show": {
                    "interact": "!send_todo",
                    "step": {
                        "w_todo": "@true"
                    }
                },
                "w_todo": {
                    "interact": "!write_todo",
                    "step": {
                        "w_note": "@valid_todo",   // jumps to "w_note" (write) when input is valid
                        "e_todo": "@invalid_todo"  // jumps to "e_todo" (error) otherwise
                    }
                },
                "e_todo": {
                    "interact": "!prompt_error_todo",
                    "step": {
                        "w_todo": "@true"
                    }
                }
                // ...yeah you get it
           }
       }
   }
   ```
    which would eventually turn into what I refer to as "high-level assembly code" in the sense that:
    1. the scheme isn't covered by a type system; that is, even if you miss spelled some "keywords" (like "entrance" or "dialog"), you will not be reminded except for receiving a runtime error
    2. it takes effort to interpret the scheme in the target language
    3. even though the patterns are abstracted, it still possesses the flaw of a configuration file - limited means of abstraction
   
    Though some shortcomings remain, the scheme file does present an abstraction that is scalable and maintainable. But we can still do better.
3. Design your own DSL. Write a parser that processes your DSL right in your target language, and start interpreting it. Good luck.

To conclude, all three options for designing FSM are either unmaintainable, unscalable, or costly.

Observation #2: A better way of capturing graph-like data structure to present a complex behavior model or relationship is needed.

Note that the complexity of such an FSM always exists; the only difference we can make is to abstract it appropriately so that some unnecessary complexity could be either hidden or avoided.

## Scenario #3: GUI design.

GUI can be declared using components and are usually cascaded. Such characteristics have aroused the idea of Object-Oriented Programming. In recent years, the idea of "components" has become well-accepted (correct me if I'm wrong), which has already done a good job on GUI abstraction. However, some dirty corners are left uncleaned:

1. The widely-used CSS files are not ideal in the sense of abstraction and data reuse. The arrival of [Sass](https://sass-lang.com/) and [tailwindcss](https://tailwindcss.com/) may suggest a thing or two.
2. UI design is strongly bonded to the coding work, which has raised the communication barrier between the programmer and the designer. Such has been the root cause of many inefficient coding cases.
3. The UI design itself is not portable. Many companies have been maintaining the same software on several platforms. Chances are that most of them share the same UI layout, but due to the strong coupling of UI design to the native code base, the same work is done several times on each platform.

Besides implementation details, some boarder problems are still debatable:

1. How to avoid the [nested hell](https://pub.dev/packages/widget_chain) in languages describing UI design (like Dart+Flutter)?
2. If we flatten the widgets, how to clearly demonstrate the flattened relationship graph?

The work of [`Cirru` and `Calcit`](https://github.com/calcit-lang) has laid effort on visualizing cascaded data, which I believe is one of the right way out.

Observation #3: As a cascaded data structure, it's hard for GUI designs to the balance among:
   1. an intuitive representation
   2. a delayed / reduced complexity on details

## Therefore...

These are all daily headaches that we can encounter a lot. These are all signs indicating that a new programming language / data notation that can swiftly manipulate data is still in great need.

// Todo: state how Lala addresses all the issues above; compare Lala with Lisp and Json; state what is **not** the goal of Lala.
