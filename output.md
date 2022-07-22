--- 
title: "在线音视频质量推断"
author: "Lin Xuhong, Email: linxuhong@yahoo.com"
date: "2022-03-15"
site: bookdown::bookdown_site
documentclass: ctexbook
bibliography: [../../OneDrive/reference/bibtex/watterry-reference.bib]
biblio-style: apalike
link-citations: yes
description: "采用自动仿真及模拟的方式，对传输算法进行验证"
---

总共分为若干章：
1. 总述，包括现有的仿真技术，以及业界动态
2. 基本系统架构
3. 用户网络模型采集
4. 用户网络模型仿真
5. 用户质量推断，即大数据分析# 总述

为什么要做用户网络仿真实验？
# 用户带宽模型采集

用灌水的方式，再用包对的方式来进行采集，那这里有一个关键问题，就是如何判断采集的准确度。

满足两个假定，一个是灌水的方式是对带宽的精确模拟，二是对随机信号相似度的判定，是有可靠方法的。

我们需要的是对网络的基本特征进行模拟，并不需要进行

## 如何判定模型采集的准确度？

本质上我们判定两个随机信号序列的相似度