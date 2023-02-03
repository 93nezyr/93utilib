class TrainerConfig:
    """
    ## About

    `class Trainer`を初期化する際に渡す設定値をまとめたクラスです。

    """

    def __init__(self, epochs, batch_size, learning_rate, device):
        self.epochs = epochs
        self.batch_size = batch_size
        self.learning_rate = learning_rate
        self.device = device

class TrainConfig:
    """
    ## About

    `class Trainer`のtrain()メソッドの引数として渡す設定値をまとめたクラスです。

    ## Fields

    """
    def __init__(self, epochs, batch_size, learning_rate, device):
        self.epochs = epochs
        self.batch_size = batch_size
        self.learning_rate = learning_rate
        self.device = device

class Trainer:
    """
    ## About

    抽象クラスである`class Model`を保持し、その学習を自動化するクラスです。

    """

    def __init__(self, config: TrainerConfig):
        pass

    def train(self, config: TrainConfig):
        pass

    def __create_train_data():
        """
        ## About

        画像ファイルを読み込んで、学習用のデータセットを生成するメソッドです。
        """
        # WIP
        pass

if __name__ == '__main__':
    print("start test")
