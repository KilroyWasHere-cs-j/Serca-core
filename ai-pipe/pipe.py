from transformers import AutoProcessor, AutoModelForVision2Seq
from PIL import Image
import torch
import requests
from transformers import BitsAndBytesConfig
from transformers import pipeline
from colorama import Fore, Back, Style

class Lava:
    def __init__(self, model_id, prompt):
        self.model = 0
        self.model_id = model_id[0]
        self.prompt = prompt[0]
        self.processor = 0
        self.device = 0
        self.image = 0

        print(Fore.BLUE + "------------------------------------------------------")
        print(Fore.GREEN + "Loading", self.model_id)
        print(Fore.CYAN + "Prompt is", self.prompt)
        print(Fore.BLUE + "------------------------------------------------------")
        print(Fore.WHITE)

    def load_model(self):
        self.processor = AutoProcessor.from_pretrained(self.model_id)
        quantization_config = BitsAndBytesConfig(load_in_4bit=True, bnb_4bit_compute_dtype=torch.float16)
        self.model = AutoModelForVision2Seq.from_pretrained(self.model_id, quantization_config=quantization_config)
        self.device = "cuda" if torch.cuda.is_available() else "cpu"
        self.model.to(self.device)

    def inference(self, image):
        self.image = image
        inputs = self.processor(text=self.prompt, images=self.image, return_tensors="pt").to(self.device) # Move inputs to GPU
        try:
            with torch.no_grad():
                outputs = self.model.generate(**inputs, max_new_tokens=512)
                return self.processor.decode(outputs[0], skip_special_tokens=True)
        except ValueError as e:
            print(Fore.RED + "Forward pass error:", e)
            print(Fore.WHITE)
            return e
        except torch.cuda.OutOfMemoryError as e:
            print(Fore.RED + "CUDA Out of Memory Error:", e)
            print(Fore.WHITE)
            return e
        except RuntimeError as e:
            print(Fore.RED + "Runtime Error during generation:", e)
            print(Fore.WHITE)
            return e