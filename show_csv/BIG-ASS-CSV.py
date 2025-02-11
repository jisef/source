import csv
import random
import string
from datetime import datetime, timedelta

def generate_random_string(length):
    return ''.join(random.choices(string.ascii_letters + string.digits, k=length))

def generate_random_date():
    start_date = datetime(2020, 1, 1)
    end_date = datetime(2025, 12, 31)
    time_between_dates = end_date - start_date
    days_between_dates = time_between_dates.days
    random_number_of_days = random.randrange(days_between_dates)
    return start_date + timedelta(days=random_number_of_days)

def generate_big_csv(filename, num_rows, num_columns):
    with open(filename, 'w', newline='') as csvfile:
        csvwriter = csv.writer(csvfile)

        # header
        header = [f'Column_{i+1}' for i in range(num_columns)]
        csvwriter.writerow(header)

        #rows
        for _ in range(num_rows):
            row = []
            for i in range(num_columns):
                if i % 4 == 0:
                    row.append(random.randint(1, 1000000))
                elif i % 4 == 1:
                    row.append(generate_random_string(10))
                elif i % 4 == 2:
                    row.append(round(random.uniform(0, 1000), 2))
                else:
                    row.append(generate_random_date().strftime('%Y-%m-%d'))
            csvwriter.writerow(row)

filename = 'big-ass-csv.csv'
num_rows = 1000000
num_columns = 5

generate_big_csv(filename, num_rows, num_columns)
print(f"CSV file '{filename}' with {num_rows} rows and {num_columns} columns has been generated.")
