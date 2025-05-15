###
### Author: Gabriel Tower
### Date: 05/15/25
###
### main function for controling video frame AI inference
### AI inference is triggered when a valid api call is made to local/process
### This code is more than likely going to cause lag spikes, unless your running on a
### 4090 super duper
### This code is mostly a servant of the rust program it dosn't make it's own decisions
###
import processor
import threading
import torch
from transformers import BlipProcessor, BlipForConditionalGeneration
from fastapi import FastAPI, Request
import uvicorn

app = FastAPI()
url = ""
target = ""
features = False

@app.post("/process")
async def process_data(request: Request):
    data = await request.json()
    print("Received:", data)
    url = data.get("url")
    # Omitted for now
    # target = data.get("target")
    # features = data.get("features")
    print("\n--------------------------")
    print("URL ", url)
    print("\n--------------------------")
    ai_data = run_processing(url)
    # Return data
    # Not everything is fully added... yet
    return { "status": "ok", "data" : ai_data, "keep-alive" : "FALSE" }

@app.get("/alive")
async def alive(request: Request):
    return { "status" : "ok" }

def main():
    serverthread = threading.Thread(target=run_server)

    serverthread.start()

    serverthread.join()


def run_processing(url):
    print("Processing images")

    blip_processor = BlipProcessor.from_pretrained("Salesforce/blip-image-captioning-base")
    blip_model = BlipForConditionalGeneration.from_pretrained("Salesforce/blip-image-captioning-base")

    yolo_model = torch.hub.load("ultralytics/yolov5", "yolov5s")

    processed = processor.process(url=url, yolo=yolo_model, blip=blip_model, blip_proc=blip_processor)
    return processed

def run_server():
    uvicorn.run(app, host="127.0.0.1", port=8000)



if __name__ == "__main__":
    main()
