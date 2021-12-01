def do_part_2():
    coord_dict = {}
    input_filename = "input"
    #input_filename = "sample_input.txt"  # Un-comment this line to use sample data instead
    y = 0
    z = 0
    w = 0

    with open(input_filename, 'r') as reader:
        for line in reader:
            this_line = str(line.rstrip())
            for x, c in enumerate(this_line):
                coord_dict[(x, y, z, w)] = c
            y += 1

    def neighbor_coord_list(coords: tuple):
        tx = coords[0]
        ty = coords[1]
        tz = coords[2]
        tw = coords[3]
        coord_list = []
        for nx in range(tx - 1, tx + 2):
            for ny in range(ty - 1, ty + 2):
                for nz in range(tz - 1, tz + 2):
                    for nw in range(tw - 1, tw + 2):
                        coord_list.append((nx, ny, nz, nw))
        coord_list.remove((tx, ty, tz, tw))
        return coord_list

    def update_map(c_dict: dict):
        neighbor_count = {}

        # Find each active coordinate, and update its neighbors
        for this_coord in c_dict:
            if c_dict[this_coord] == '#':
                neighbor_list = neighbor_coord_list(this_coord)
                for this_neighbor in neighbor_list:
                    # If this neighbor hasn't been added, add it
                    try:
                        neighbor_count[this_neighbor] += 1
                    except KeyError:
                        neighbor_count[this_neighbor] = 1

        n_map = {}
        active_count = 0

        # Go through each coord with active neighbors and see if it needs to change its state
        for this_coord in neighbor_count:
            # If the coord is newly viewed, then its initial state must be inactive.
            try:
                current_state = c_dict[this_coord]
            except KeyError:
                current_state = '.'

            # Active coordinates stay active only if they have 2 or 3 active neighbors
            if current_state == '#' and not (2 <= neighbor_count[this_coord] <= 3):
                n_map[this_coord] = '.'
            # Inactive coordinates become active if they have exactly 3 active neighbors
            elif current_state == '.' and neighbor_count[this_coord] == 3:
                n_map[this_coord] = '#'
            # Otherwise, their state remains unchanged
            else:
                n_map[this_coord] = current_state

            # Up the count if this coordinate is active
            if n_map[this_coord] == '#':
                active_count += 1

        return n_map, active_count

    for i in range(6):
        coord_dict, counter = update_map(coord_dict)
    
    print(counter)

do_part_2()