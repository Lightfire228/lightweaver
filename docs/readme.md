
# Lightweaver

LW is a program that takes a script, and converts it into a diagram
- a visual representation of your mental model


# Features

## > Language

- should be expressive
    - some form of user defined object structure (`struct GraphNode {}` / `impl GraphNode {}`)
        - and individual object instances (`GraphNode::new()`)
    - as expressive as a basic scripting language
    - capable of runtime data manipulation
        - even the graph data itself (reflection)
    - arrays and iterators
    - control logic
        - if
        - while
    - functions
        - such as "thunks" that can be used to dynamically return runtime generated parts of the graph
    - should be able to express some control over layout engine
        - such as vertical alignment vs horizontal alignment
        - defining anchor points and connections
        - 

- can take a data dump (of some specified format)
    - and convert it into a graph
    - for vizualizing
        - network request logs
        - SQL Queries
        - 
- 
- or maybe just an actual API?
    - language bindings?


- Ability to define data / graph model 
    - And separately, define what gets rendered
        - (analogous to)
            - a view or slice into the data
            - A Projection or Select statement

    - this allows the user to 
        - show only relevant details
        - or render multiple, independent views 
            - from a single data model



## > UI/UX

- should be able to see instant feedback of the graph updating
    - autorefresh
        - only showing the last successful compilation, if there is a syntax error
    - on demand
        - cli command
            - cumbersome
        - using an editor keybinding 
            - (like a run button)


- it should be easy to test out ideas visually
    - should be as little friction as possible between the language and generating a diagram
        - (i'm still not sure what requirements this needs to satisfy, yet)

- should be a C-like lang
    - inspirations
        - python
        - [HCL](https://github.com/hashicorp/hcl-lang)
    - with support for "first class nodes"?
### >> Autorefresh

ideally, provides instant feedback, like a language LSP

## > Vizualizations

- take a network request log dump and convert it into a sensible graph
- how data flows through a complex ETL application
- an ancestry tree
    - mother 
    - father 
    - divorces
    - step-kids
    - out-of-wedlock births
- 
- 
- 
- 

## > Uses
- software design
- reverse engineering 
- mental model visualizations
- 