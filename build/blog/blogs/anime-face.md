# Automatic Anime Face Generator
*Dec 2, 2019*

I suck at drawing. But I’m good at designing AI networks, meaning I’m actually really good at drawing.

![The Result](../images/gan-anime.jpg)

Just make sure not to look too closely...

So what I did here is really self-explanatory. I feed in a bunch of data from really good artists into an AI called a deep convolutional generative adversarial network (DCGAN), and the network learned to reuse the features of the faces, like hair and eye shape but in different combinations to synthesize new images.

## How it actually works
Fundamentally, there are two main parts to arrive at this fully trained network. First, you need to somehow collect a large dataset of well-drawn out pictures. The higher the quality the better. And then second, you need to simultaneously train two neural networks on this data: a generator that captures the distribution of the data, and a discriminator (classifier) that estimates the chance of a given image being part of the real dataset rather than coming from the generator.

## Collecting the data
So let's start with the first part, where we have to collect a dataset of top-quality anime face drawings. The secret here is that the data was collected by web scrapping from a Japanese anime website called ‘getchu.com’.

Web scrapping basically means automating away the task of collecting information, in this case anime faces, from websites. I wrote a script to acquire and parse the images from the raw HTML of the web pages, and then applied an anime face detector called lbpcascade-animeface on each image to get a bounding box for the faces.

I then rescaled the images to be 64x64 pixels and only used images from after 2010 since they were of the highest quality.

---

![Bounding Box Example](../images/bounding-box.jpg)

And then finally, with around 20 000 images in the dataset, I went in manually and removed some of the undesired images.

## Building and training the AI

So I used a DCGAN, that we can kind of imagine as a mix of a convolutional neural network (CNN) and a generative adversarial network (GAN).

#### CNN

What the CNN does is quite simple since all it does is recognize and store the features of an image, in this case, the hair, face, and eye shapes as well as colors of the anime faces.

It takes in a 3d tensor with dimensions 3x64x64, which represents the three color channels of the images (red, green blue) and the resolution (64x64) and passes the values through a bunch of convolutional layers, which are made up of:

1- A 2d convolutional transpose, which is a math operation that looks like this:

![Convolution Animation](../images/conv.gif)

2- A batch normalization, which you can imagine as taking the input to the layer and then transforming then so they hover around 0 with a standard deviation of 1, like in this image:

![Batch Normalization](../images/batch-norm.jpg)

3- And then a ReLU activation function to get rid of the negatives:

![ReLU Function](../images/relu.jpg)

#### GAN

The GAN acts as a framework to capture the training data’s distribution so we can generate new data from that same distribution. In a GAN, there is a generator and a discriminator. The job of the generator is to synthesize fake images that look like the training data. The job of the discriminator is to look at an image and output whether it is a real training image or a fake from the generator.

These two enter in a competition, where the generator gets better at outsmarting the discriminator, and the discriminator gets better at spotting the fakes. The ideal ending of this competition, what is what we call the ‘equilibrium’ or ‘solution’ and its the point where the discriminator always guesses 50/50 that the image shown is real (since the generator becomes perfect).
Image for post

![Discriminator](../images/discriminator.jpeg)

At the end of the training, you can throw out the discriminator and use the generator to synthesize anime face images!

## Key Takeaways
- Using a DCGAN with a whole bunch of data, you can make an automatic generator of whatever you want.
- You can collect that data from online using web scrapping.
- The generator and discriminator are competing until the generator becomes perfect and the discriminator has to guess if the image is real or fake.
