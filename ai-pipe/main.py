# main.py
# TODO Switch to ffmpeg
from colorama import Fore, Back, Style
import cv2
from llava.model.builder import load_pretrained_model
from llava.mm_utils import process_images, tokenizer_image_token
from llava.constants import IMAGE_TOKEN_INDEX, DEFAULT_IMAGE_TOKEN
from llava.conversation import conv_templates
from PIL import Image
import copy
import torch
import warnings
from halo import Halo

path = "descriptions.txt"
spinner = Halo(text='Processing frames', spinner='dots')

warnings.filterwarnings("ignore")

pretrained = "AI-Safeguard/Ivy-VL-llava"

model_name = "llava_qwen"
device = "cuda"
device_map = "auto"
tokenizer, model, image_processor, max_length = load_pretrained_model(pretrained, None, model_name, device_map=device_map)  # Add any other thing you want to pass in llava_model_args

model.eval()

conv_template = "qwen_1_5"
question = DEFAULT_IMAGE_TOKEN + "\n" + "What's in this image"
conv = copy.deepcopy(conv_templates[conv_template])
conv.append_message(conv.roles[0], question)
conv.append_message(conv.roles[1], None)
prompt_question = conv.get_prompt()

def process_frames(frames):
    output = ""
    spinner.start()
    print("Processing frames")
    print("\n")
    for frame in frames:
        image_tensor = process_images([frame], image_processor, model.config)
        image_tensor = [_image.to(dtype=torch.float16, device=device) for _image in image_tensor]

        input_ids = tokenizer_image_token(prompt_question, tokenizer, IMAGE_TOKEN_INDEX, return_tensors="pt").unsqueeze(0).to(device)
        image_sizes = [frame.size]

        cont = model.generate(
            input_ids,
            images=image_tensor,
            image_sizes=image_sizes,
            do_sample=False,
            temperature=0,
            max_new_tokens=4096,
        )

        output = tokenizer.batch_decode(cont, skip_special_tokens=True)
    spinner.succeed(text="We processed the fuck outa that!")
    return output

def grab_frames(url, frame_snaps):
    video_url = url
    frame_inc = frame_snaps
    frame_count = 0
    frames = []
    # Initialize video stream
    cap = cv2.VideoCapture(video_url)
    print("Caching videos")
    print(video_url)
    print(video_url)
    while True:
        print(frame_count)
        # Capture frame-by-frame from the video
        ret, frame = cap.read()
        # If a frame is read correctly, ret will be True
        if not ret:
            print("Failed to grab frame")
            break

        if frame_count % 15 == 0:
            rgb_frame = cv2.cvtColor(frame, cv2.COLOR_BGR2RGB)
            pil_image = Image.fromarray(rgb_frame)
            frames.append(pil_image)
        frame_count += 1

        if cv2.waitKey(1) & 0xFF == ord('q'):
            break

    print("All cached")
    # Release the video capture object
    cap.release()
    cv2.destroyAllWindows()
    print(len(frames))
    return frames

def push_into_file(url, description):
    file1 = open(path, "a")
    file1.write("\n")
    file1.write("--")
    file1.write(url)
    file1.write("\n")
    file1.write(str(description))
    file1.close()

#  Follow these links at your own risk, no malware just odd ass videos
def main():
    urls = ["https://a.nwps.fi/1402041709527.webm", "https://a.nwps.fi/15301210957112.webm"]
    for url in urls:
        # Grab frames from the video at url
        frames = grab_frames(url, 15)
        print("Total frames")
        print(len(frames))
        description = process_frames(frames=frames)
        push_into_file(url=url, description=description)
    
if __name__ == "__main__":
    main()