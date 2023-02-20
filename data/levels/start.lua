local function test (inputs)
    return inputs
end

local table = {true, false, nil}

local function generate(i)
    return table[i]
end

level.test = test
level.generate = generate
level.inputs = { "bit" }
level.outputs = { "bit" }

level.name = "Start"
level.description = [[
Thank you for purchasing MarbleCorp (tm)'s flagship product, `M.O.D.U.L.E.S`!
With these, you'll be able to satisfy all of your computing needs.

---

But don't get ahead of youself, you have to learn the basics. In each level there
are input and output modules. The controls should be pretty intuitive.

Click on a module to select it, your possible actions will be displayed in the bottom
left, and small widgets will appear on the module which you can use to
interact with it.

A tracer will appear to show the trajectory of any fired marble. Use this to help 
position any modules you place.

You can place modules by selecting one on the left panel, and clicking where you want it
to go. You cannot place a module intersecting another one.

---

Your objective for this level is simple; Get the marble from the input module, to the
output module.
]]
