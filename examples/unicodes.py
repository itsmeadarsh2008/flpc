import flpc as re
re.match = re.fmatch
compiled_regex = re.compile('.*')
print(re.match(compiled_regex,'hello \N{EARTH GLOBE AMERICAS}').span(0))