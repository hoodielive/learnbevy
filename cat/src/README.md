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
