from transformers import LlavaProcessor, LlavaForConditionalGeneration
import torch
model_id = "llava-hf/llava-interleave-qwen-0.5b-hf"

processor = LlavaProcessor.from_pretrained(model_id)

model = LlavaForConditionalGeneration.from_pretrained(model_id, torch_dtype=torch.float16)
model.to("cuda") # can also be xpu, mps, npu etc. depending on your hardware accelerator

import uuid
import requests
import cv2
from PIL import Image

def replace_video_with_images(text, frames):
  return text.replace("<video>", "<image>" * frames)

def sample_frames(url, num_frames):

    response = requests.get(url)
    path_id = str(uuid.uuid4())

    path = f"./{path_id}.mp4" 

    with open(path, "wb") as f:
      f.write(response.content)

    video = cv2.VideoCapture(path)
    total_frames = int(video.get(cv2.CAP_PROP_FRAME_COUNT))
    interval = total_frames // num_frames
    frames = []
    for i in range(total_frames):
        ret, frame = video.read()
        pil_img = Image.fromarray(cv2.cvtColor(frame, cv2.COLOR_BGR2RGB))
        if not ret:
            continue
        if i % interval == 0:
            frames.append(pil_img)
    video.release()
    return frames[:num_frames]

video_1 = "https://huggingface.co/spaces/merve/llava-interleave/resolve/main/cats_1.mp4"
video_2 = "https://huggingface.co/spaces/merve/llava-interleave/resolve/main/cats_2.mp4"

video_1 = sample_frames(video_1, 6)
video_2 = sample_frames(video_2, 6)

videos = video_1 + video_2

videos

# [<PIL.Image.Image image mode=RGB size=1920x1080>,
# <PIL.Image.Image image mode=RGB size=1920x1080>,
# <PIL.Image.Image image mode=RGB size=1920x1080>, ...]

user_prompt = "What is going one in this video?"
toks = "<image>" * 12
prompt = "<|im_start|>user"+ toks + f"\n{user_prompt}<|im_end|><|im_start|>assistant"
inputs = processor(text=prompt, images=videos, return_tensors="pt").to(model.device, model.dtype)

output = model.generate(**inputs, max_new_tokens=100, do_sample=False)
print(processor.decode(output[0][2:], skip_special_tokens=True)[len(user_prompt)+10:])

# The first cat is shown in a relaxed state, with its eyes closed and a content expression, while the second cat is shown in a more active state, with its mouth open wide, possibly in a yawn or a vocalization.

