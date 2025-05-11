import torch


class Yolo:
    model = ''
    results = ''

    def __init__(self):
        self.model = torch.hub.load("ultralytics/yolov5", "yolov5s")
        print("Loaded Yolov5")

    def inference(self, image):
        self.results = self.model(image)

    def r_print(self):
        self.results.print()

    def r_show(self):
        self.results.show()

    def r_save(self):
        self.results.save()
