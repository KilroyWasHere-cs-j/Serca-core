import server
import processor
import threading
import torch
from transformers import BlipProcessor, BlipForConditionalGeneration


def main():
    # serverthread = threading.Thread(server.start_server())
    processingthread = threading.Thread(run_processing())

    # serverthread.start()
    processingthread.start()

    # serverthread.join()
    processingthread.join()


def run_processing():
    print("Processing images")
    url = "https://a.nwps.fi/5minutescraft.mp4"

    blip_processor = BlipProcessor.from_pretrained("Salesforce/blip-image-captioning-base")
    blip_model = BlipForConditionalGeneration.from_pretrained("Salesforce/blip-image-captioning-base")

    yolo_model = torch.hub.load("ultralytics/yolov5", "yolov5s")

    processor.process(url=url, yolo=yolo_model, blip=blip_model, blip_proc=blip_processor)

if __name__ == "__main__":
    main()
