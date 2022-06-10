function StringToBool(s: string) : bool
{
    if(s == "false" || s == "" || !s) {
        return false;
    } else {
        return true;
    }
}

function BoolToString(b: bool) : string
{
    if(b) {
        return "true";
    } else {
        return "false";
    }
}