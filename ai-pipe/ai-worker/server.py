from fastapi import FastAPI, Request
import uvicorn

app = FastAPI()

@app.post("/process")
async def process_data(request: Request):
    data = await request.json()
    print("Received:", data)
    return {"status": "ok"}

if __name__ == "__main__":
    uvicorn.run(app, host="127.0.0.1", port=8000)
