# Structure of Game

## Component(s)
1. Attack: attack power and range
2. Transform: keep track of location, orientation, and scale
3. Collision: detect collisions
4. Health: keep track of health and death

## Entities
1. Player: Attack + Transform + Collision + Health
2. Enemy: Attack + Transform + Collision + Health
3. Tree: Transform + Collision

## Systems
1. Movement: move the entities and update their transform. 
2. Input: takes user input and updates the player's location and perform attacks
3. Collision: checks for collision and stop the entities from crossing each other, incur damage
4. Attack: in lieu of attack, reduce health of victim based on the attacker's attack power.

## Explanation
Systems can be based on single components, or more often will deal with an interaction between components. For example, the collision system would need to look at entities' transform  components to determine when components collide, and may also need access to each entity's health component to handle any damage from the collision.

Using this architecture, we can make our code clean and structured, which will aid in the construction of complicated games. Creating a new novel entity often is a simple as giving it a quick combination of components that already handle the required behavior. If new behavior is desired, a new component can be created fo rthe entity and then be reused in the future when other new entities use the same behavior. Just like in any form of programming, this leads to having to make decisions about how specialized or generic your components will be. For simple projects, specialized components and some duplication is OK, but as your project grows its easier to use common components and simply change how they are combined.
