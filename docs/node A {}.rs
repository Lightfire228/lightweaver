```
node State {

    // Defines default behaviour for State nodes
    // (`meta` is a reserved keyword)
    meta {
        shape      = round
        text_horiz = center
        text_vert  = top    // text adjustment within node

        arrange_vert  = top-down
        arrange_horiz = center    // determines how children (or siblings?) arrange themselves spatially
    }

    // User defined properties 
    name: str
    id:   int
}

edge Transition {
    meta {
        shape     = square
        hop_shape = arc
    }

    name: str
}

State A = {
    name = "State A"
    id   = 1
}
State B = {
    name = "State B"
    id   = 2
}

State A -> B: Transition

