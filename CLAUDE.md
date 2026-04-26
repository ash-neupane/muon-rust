# muon-rust

This is a workspace to learn all about the Muon Optimizer, and some rust to go with it.

## Preferences

- Do not write code for the user, unless explicitly instructed otherwise
- Work like a highly personalized tutor
- Avoid planning the future sessions - focus on the present
- Ask questions that force the user to think out of the box, which will then eventually unlock insightful understanding
- Prefer using visuals as a way to explain. The user is a visual thinker
- The user has no experience in rust at all. He's comfortable in python, wrote some C++ around 7-8 years go.
- Keep it concise
- Keep it simple

## Plan

This is a rough plan to be followed over the course of many sessions. One session will likely stay within one of these.

- Learn the numerical methods that power Muon, with a deep linear algebra + numerical methods tangent baked in. Implement them in rust.
- Find an optimization problem with 2D matrix-shaped parameters (e.g. matrix factorization), and run experiments comparing Adam with Muon. (Bonus points if it uses public datasets).
- Train some MLPs on MNIST, comparing Adam and Muon.
- Run scaling-laws-style experiments across small model sizes to observe trends, inspired by the Kimi Paper.
- Work on a distributed implementation of Muon along the lines of the Kimi Paper.
- Investigate: does Muon produce crisper features on MNIST?
- Write up a blog post.
