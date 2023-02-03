import torch
from model import Model

class MobileNet(Model):
    def __init__(self):
        self.model = torch.hub.load('pytorch/vision:v0.10.0', 'mobilenet_v2', pretrained=True)
        self.model.eval()

    def train(self):
        pass

    def eval(self):
        pass



if __name__ == '__main__':
    print("start test")