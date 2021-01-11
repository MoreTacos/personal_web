# Deep Learning for the Origins of Life
*Jan 11, 2021*

My goal is to cure ageing, and simulating a cell is an important milestone for that goal. My hypothesis is that were we to understand abiogenesis (the process by which chemicals evolve into biology), we would be well on our way to simulating a cell.

Experimentally going through a trial and error search for the 'recipe' for life is a hard task for humans, but might be easier for AI. We can get a network to simulate the evolution of a multitude of initial molecule systems, and see if anything interesting pops up. How exactly would we do that?

One place to look is deep learning for fluid simulations. In this field, fluids are accelerated using reduced-order models, while respecting the convergence constraints provided by higher quality models.

When it comes to simulation, we are lucky in terms of machine learning. We are lucky because ML requires a massive amount of data, and in simulation fields, we can generate it; we simply need to use higher quality models to calibrate our network.

In natural language processing, an RNN predicts the next word in a sentence by looking at the previous words. The same could be done with simulations of chemical systems. We can apply the network to predict the system's state at the next timestep. The system's physics is encoded into the network's parameters.
