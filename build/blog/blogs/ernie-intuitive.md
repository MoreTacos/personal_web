# The intuition for ERNIE 2.0 — Language Understanding
*Sep 6, 2019*

Natural language processing (NLP) has been a leading sub-field of machine learning, with pre-trained networks showing dominant results in various language understanding tasks. Part of this sub-group is a recently published paper from the researchers at Baidu, who propose a brand new framework architecture named ERNIE 2.0 that has achieved state-of-the-art performance. In this text, we will provide intuition behind the core concepts of ERNIE 2.0 in addition to a solid framework for understanding the newest developments in natural language processing.

# The Dilemma

In regular supervised neural networks, data is labeled and used to mathematically adjust the parameters of the neurons to maximize performance. But in NLP, a dilemma is presented where near-endless amounts of non-labeled information exist on the internet in the form of articles, encyclopedias, news, dialog and other language data available, but cannot be used when applying traditional techniques. This has lead to a surge in research to find new ways of extracting as much useful information from unlabeled data as possible, in particular by employing co-occurrence probability — the chance words appear beside one another — in a technique called unsupervised learning, which fortunately does not require labels. Following this session of ‘unsupervised general language learning’, where the network has hopefully learned the basics of the language, a more thorough supervised phase of learning called ‘fine-tuning’ takes place, which uses smaller labeled datasets to finalize training and specialize the network in tasks like:

- Sentiment analysis
- Name-entity recognition
- Question answering
- Translation

We give the name ‘pre-training’ to the previous unsupervised phase of learning, and this is the section of natural language processing that has progressed rapidly and allowed language understanding networks to perform increasingly better as pre-training architectures improve.

# Core Ideas of ERNIE 2.0

The researchers propose a new pre-training architecture called ERNIE 2.0 which they claim allows them to achieve state of the art results.

## Idea 1
Increase information extraction from the unlabeled data during the unsupervised phase of learning

Problem: Current pre-training procedures usually focus on pre-training the model with several simple tasks to grasp the co-occurrence of words and sentences. However, besides co-occurring, there exists other valuable information that can be extracted from the data that is being left behind.

Solution: The researchers propose expanding pre-training tasks to include more than just co-occurrence of words and sentences, but also tasks like:

- A letter capitalization prediction task, in which the network is shown certain words like ‘paris’ and ‘cat’ and asked which one should be capitalized, in this case being ‘Paris’.
- A document-word relationship pattern, where a sentence like: ‘Cats are predominantly active in the night’ is shown, and the document is asked which word is most likely to be repeated in other parts of the same article, the answer to this example being ‘cat’.
- A sentence reordering task to learn the relationship among sentences. The sentences of a paragraph are split and shuffled randomly. The network is then asked to piece it back together as a paragraph.

## Idea 2
Reduce the loss of learned progress when changing tasks.

Problem: Current pre-training architectures tend to forget information when moving on from a task to another. Like when a ship is leaking from multiple holes, a network will go and adjust parameters to satisfy the first task — plug the leak — and then when encountering the second task it will adjust its parameters, causing it to perform well on the new task but at a risk of unlearning information gained previously — Like moving the same hand from the first leak to the second, causing the first to leak once more.

Solution: The researchers propose the use of a technique called: multi-task learning, in which during the unsupervised phase of the training, tasks are introduced incrementally in a way that penalizes ‘unlearning’ information. Specifically, the network is asked to:

1. Perform and adjust parameters to maximize results on Task 1.

2. Perform and adjust parameters to maximize results on Task 1 and Task 2 at the same time.

3. Perform and adjust parameters to maximize results on Task 1, Task 2 and Task 3 at the same time.

---

# Conclusion

To summarize, a double approach is used when training neural networks in the field of natural language processing, to take full advantage of both the massive amounts of unlabeled data available online during unsupervised learning in ‘pre-training’, and of the benefits of using traditional labeled data for supervised learning during ‘fine-tuning’. ERNIE 2.0 is one of many new architectures that innovates in the two major areas of increasing information extraction from the unlabeled data during the unsupervised phase of learning by expanding the definition of pre-training tasks, and in reducing the loss of learned progress when changing tasks by using multi-tasking.

If you want to know more about the nitty-gritty architecture of ERNIE 2.0 as well as its results compared to other pre-training frameworks, feel free to read the paper here:

Paper: ERNIE 2.0: A Continual Pre-training Framework for Language Understanding

Link: https://arxiv.org/abs/1907.12412