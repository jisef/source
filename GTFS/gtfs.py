import pandas as pd

# Define the GTFS folder path
gtfs_folder = "/Users/josef/Downloads/gtfs"

# Load the files into DataFrames
routes_df = pd.read_csv(f"{gtfs_folder}/routes.txt")
trips_df = pd.read_csv(f"{gtfs_folder}/trips.txt")

# Adjust Pandas options to display all rows and columns
pd.set_option('display.max_rows', None)  # Display all rows
pd.set_option('display.max_columns', None)  # Display all columns
pd.set_option('display.expand_frame_repr', False)  # Prevent wrapping of columns

# Print the content of the files
print("Routes Data:")
print(routes_df)

print("\nTrips Data:")
print(trips_df)
