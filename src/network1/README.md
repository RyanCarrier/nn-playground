# Pt 1

Making 2 input and implementing AND
Then go to 3 and add or


prior knowledge makes the current plan;

3 inputs, 1 output

4? internal nodes with 2 layers?
 
 - I know we need more internal nodes than inputs, for combination inputs
 - we would also need multiple layers to then handle those combination inputs no? (like if you had first 2 bits are for and/or and second 2 are for inputs, you would need combination first 2 bits to know how to handle second 2 bits)
 - though with that it feels like layers is context dependant, and not actually node dependant...

Learning rate can scale linearly it will be fine

## pt2

Ok clearly not working and I thikn big issue is from wanting binary outputs with ananlogue inputs;

In future you will need things to decode inputs into input nodes (like pixels or shape coordinates??), but potentially also for output too. So having a analogue to binary converter is probably 'ok' and 'normally' done? idk there is transformers and diffusers and stuff, one of those is probably output

plus i know matricies are big shit for nn so probably need to drop the layers stuff anyway for matricicies somehow; matricies good cause SIMDeeznuts or something like that


## pt3

ok having defualt paths of 1 works... but that's only because the default works. I think it's good having all paths be worth something to start with?

so making a part 1.5 to do AND, cause the default won't work for AND (before moving to pt 2 which is And and Or)

OK we can't do it with how it is currently cause you need some value to be injected somehow for 0,0 to make 1. So we could add an input that is controlled by weighting (but this feels wrong, becuase then it feels like for complex cases you would be probably doubling the input? with arbitrary info?), OR we could move the value of nodes up or down manually? which would be best for memory, but it feels wrong so we are probably suppose to move the paths up and down, but then idk how this would work for going to matricies...

We will try with adding constant to nodes

I also have no idea what my internal bounds should be, [-1,1] at least... but why not no bounds? feels like being able to go infinite on weights could throw is into wrong local max's or something

### pt 3.5

I think i was kinda wrong about output transform, going anologue to binary means you can't slowly ppush in right direction, but we also don't need it landing exactly on the 1.0 cause that will be hard so maybe we will (expect) bounds to be [0,1] but scale it for the result so ((result*2)^2)/2 , this shouldn't be too heavy

no that won't work lol

we will need sin so some kind of e^x-e^-x or smth, or smth 1- 1/1+x 

ok the google sheets mini taylor series was a failed plan for looking at -sin

ok played with graph for (-2x3+3x2), but honestly linear with bounds is probably the way to go lol

### pt 3.6

Got original one working properly, a few things; c wasn't getting reset oops

Wasn't saving weights properly/reverting them oops (currently we are doing it too often but who cares)

changed output transform to just min max 1.0,0.0

## pt 4

I don't even know, just gave it some layers and nodes and it just worked tbh

## pt 5

Can go to matricies by not storing value in each, cause value would be returned, then would just add to matricie, that's how
