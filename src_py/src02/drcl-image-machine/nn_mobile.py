import torch

class MobileNet:
    def __init__(self):
        self.model = torch.hub.load('pytorch/vision:v0.10.0', 'mobilenet_v2', pretrained=True)
        self.model.eval()


if __name__ == '__main__':
    print("start test")