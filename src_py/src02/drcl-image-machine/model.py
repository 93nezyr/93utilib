import abc

class Model(metaclass = abc.ABCMeta):
    """
    ## About

    ニューラルネットワークとそれに付随する機能を持つクラスの抽象基底クラスです。

    """

    @abc.abstractmethod
    def train():
        raise NotImplementedError

    @abc.abstractmethod
    def eval():
        raise NotImplementedError

