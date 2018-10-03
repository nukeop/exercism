def hey(phrase):
    phrase = phrase.strip()

    if len(phrase) < 1:
        return "Fine. Be that way!"

    if phrase.isupper() and phrase [-1] == "?":
        return "Calm down, I know what I'm doing!"
    
    if phrase.isupper():
        return "Whoa, chill out!"
    
    if phrase[-1] == "?":
        return "Sure."

    return "Whatever."
