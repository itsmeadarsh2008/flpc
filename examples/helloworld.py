import flpc

# Compile a pattern
pattern = flpc.compile(r'(\w+) (\w+)', flags=0)

# Search for the pattern in text
match = flpc.search(pattern, "Hello World")
if match:
    print("Group 0:", match.group(0))  # Hello World
    print("Group 1:", match.group(1))  # Hello
    print("Group 2:", match.group(2))  # World
    print("Groups:", match.groups())   # ['Hello', 'World']
    print("Start of group 0:", match.start(0))  # 0
    print("End of group 0:", match.end(0))  # 11
    print("Span of group 0:", match.span(0))  # (0, 11)

# Find all matches
matches = flpc.findall(pattern, "Hello World Hello Python")
print("All matches:", matches)  # ['Hello World', 'Hello Python']

# Find all matches with iterator
matches_iter = flpc.finditer(pattern, "Hello World Hello Python")
for match in matches_iter:
    print("Match:", match.group(0))  # Hello World, Hello Python
    print("Groups:", match.groups())  # ['Hello', 'World'], ['Hello', 'Python']

# Replace matches
replaced_text = flpc.sub(pattern, r'\2 \1', "Hello World")
print("Replaced text:", replaced_text)  # World Hello