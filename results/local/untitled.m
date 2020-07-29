transaction_sent = [10, 100, 1000];
transaction_times = [0.568, 4.347, 43.517];

plot(transaction_sent, transaction_times)

file = readmatrix('transaction_processing_duration.csv');