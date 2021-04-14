fileobj = open("test.sh", "r", encoding="utf-8")
file_num = 1
while True:
    line = fileobj.readline()
    if line:
        if 'assert "main' in line:
            first = line.find('"')
            second = line.find('"', first + 1, 1200)
            write_data = line[first+1:second]
            file_name = "test" + str(file_num) + ".txt"
            file = "./test/" + file_name
            f = open(file, 'w', encoding='utf-8')
            f.write(write_data)
            f.close()
            file_num += 1
    else:
        break
