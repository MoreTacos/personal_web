# Simulating the Origins of Life
*Jan 11, 2021*

My hypothesis is that if we could understand how chemistry evolve into biology, we would be on our way toward simulating a cell.

Experimentally going through a trial and error search for the 'recipe' for life in a lab is a hard task for humans. But would be much easier if first simulated. We can simulate the evolution of a multitude of initial molecule systems, and see if anything interesting pops up. How exactly would we do that?

One place to look is deep learning for fluid simulations. In this field, fluid simulations are accelerated using reduced-order models, while respecting the convergence constraints provided by higher quality models. You can get a realistic simuation even while dropping most information.

We are lucky when it comes to machine learning for simulation in general. We are lucky because getting data is easy. All we need is to use higher quality models to generate data to calibrate our network.

I hope is that a similar abstraction can be achieved for molecular dynamics.

In natural language processing, an RNN predicts the next word in a sentence by looking at the previous words. The same could be done with simulations of chemical systems. We can apply the network to predict the system's state at the next timestep. The system's physics is encoded into the network's parameters.

Another place to look is quantum level simulations. In the case that low level information is truly important for accurate molecular dynamics simulations, quantum computers could be the salvation of the idea. I have heard, but not investigated that quantum computers are much better at simulating quantum phenomenon then traditional computers.

Whetever the technique we end up using, the idea of using atomistic simulations of molecules to find the origins of life is the main path I am intersted in. Eventually leading to the *perfect* simulate of a whole cell.
