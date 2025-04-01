# main.py
import utils
import pipe
import torch
from pipe import Lava

def main():
    # processor, model, device = pipe.load_model()
    # image = pipe.load_image("bagel.jpg")
    # pipe.inference(processor=processor, device=device, model=model, image=image)

    image = "bagel.jpg"
    output = "NULL"

    model_id, prompt = utils.load

    lava = pipe.Lava()
    lava.load_model()
    lava.load_image(image)
    output = lava.inference()
    print(output)
    
if __name__ == "__main__":
    main()
