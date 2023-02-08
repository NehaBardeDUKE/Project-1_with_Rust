## Information redaction microservice based on IAM

Based on the person requesting the information, the raw data or a summarized data is displayed.

Possible usecase: FERPA requires students to be non identifiable and only the students to have access to their educational info. So if a mental health professional needs a student's info the microservice will provide required statistics (like student's performance metric) instead of the actual data.

## Usage

To build the container and run the application:
```bash
docker pull nehabardeduke/redact_ids:latest
docker run --rm -it redact_ids start 
```

## Basic Idea/Architechture
![image](https://user-images.githubusercontent.com/110474064/217631977-e689b521-bda7-4ea6-8aa1-b154f3080b83.png)




