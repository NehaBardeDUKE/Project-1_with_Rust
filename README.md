## Information redaction microservice based on IAM

Based on the person requesting the information, the raw data or a summarized data is displayed.

Possible usecase: FERPA requires students to be non identifiable and only the students to have access to their educational info. So if a mental health professional needs a student's info the microservice will provide required statistics (like student's performance metric) instead of the actual data.

## Usage

To build the container and run the application:
```bash
docker build -t redact_ids .
docker run --rm -it redact_ids start 
```



