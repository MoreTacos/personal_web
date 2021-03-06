# Learn Vulkan by Diving Into the Specs
*Nov 11, 2020*

A few weeks ago, I started learning about [Vulkano](https://github.com/vulkano-rs/vulkano), which is a safe Rust wrapper around the Vulkan graphics API. I debated whether starting with OpenGL was a better choice, but stuck with Vulkan in the end because it better reflects the process of running programs on graphics hardware architectures.

![Vulkan Custom Logo](../images/vulkan.jpg)

Learning OpenGL is not the best thing to do if you wish to learn solid graphics programming, because it doesn't really explain what's going on. Rather, all these books and tutorials teach by example and quickly show how to do something, but not what that something means. And honestly I think it's a good thing they don't explain the background! OpenGL is inherently confusing because it tries to support all features and new graphics programming paradigms from the last two decades. There are many ways to do many things, and in the case when a feature isn't supported, it silently falls back to software emulation, or worse just flags an error and refuses to work.

Vulkan on the other hand is extremely verbose, where everything you do has a purpose. The more the complexity of your program rises, the more valuable Vulkan becomes. This is because Vulkan is more consistent. However this extreme detail comes at a cost, the downside being that it presents a massive learning curve and everything must be setup from scratch.

In my experience as a beginner going through the Rust version of the vulkan-tutorial, whenever I encountered a problem, my first reaction was to turn to reddit or stack-overflow and find an explanation. This is because I though the actual specification or vulkano documentation would surely be arcane and filled with accurate but unhelpful explanations and legal jargon. For example at one point I was very confused about the role render passes play in rending vertices onto the screen, and no suitable answer was to be found on reddit/stack overflow.

The reason questions like these can't be found on reddit/stack overflow is because they have already been answered in the Vulkan specs. And contrary to what I though, it is in fact very approachable and helpful. I've trained myself to use the specs much more frequently when I don't understand a feature. I find the glossary at the end to be particularly useful for quick recaps. My methodology for understanding a concept is as follows:



1. Read the main explanation in the table of contents

2. Highlight and search all words in the explanation I don't understand

3. Rinse and repeat until I've reduced the concept to its basics (recursive)

4. Slowly start building everything back up again from the group up

Iteration 1:

A render pass represents a collection of **attachments**, **surpasses**, and dependencies between the surpasses, and describes how the attachments are used over the course of the surpasses.

Iteration 2:

Attachment (Render Pass): A zero-based integer index name used in render pass creation to refer to a **framebuffer** attachment that is accesed by one or more subpasses. The index also refers to an attachment description which includes information about the properties of the **image view** that will later be attached.

A subpass represents a phase of rendering that reads and writes a subset of the attachments in a render pass. Rendering commands are recorded into particular subpasses of a render pass instance.

Iteration 3:

Framebuffer: A collection of **image views** and a set of dimensions that, in conjunction with a render pass, define the inputs and outputs used by drawing commands

Image View: An object that represents an **image subresource range** of a specific image, and state that controls how the contents are interpreted.

Iteration 4:

Image subresource range: A set of image subresources that are contiguous mipmap levels and layers

Image subresource: A specific mipmap level and layer of an image

Image: A resource that represents a multi-dimensional formatted interpretation of device memory

---

Perfect! The concept has been fully reduced to components I believe I understand. For reference this is a mipmap:

![Example of a MipMap](../images/mipmap.jpg)

Now that the concept has been reduced to its bases in a tree-like structure, I can slowly go back up the tree, making sure I fully understand each component intuitively.

In fact another technique I use to seal the deal and **fully** understand, is that I then try to pass on the explanation to somebody else (usually my brother or dad if they're around). The process of organizing your thoughts in your head is superbly helpful for intuition. It's of my opinion that the best way to learn is to teach. Kind of like this blog post is doing come to think of it..!
