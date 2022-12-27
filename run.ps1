$lang = $args[0];
$demo = $args[1];

if ($args.Count -eq 1) {
    $demo = $lang;
    $lang = "rust";
}

if ($lang -eq "rust") {
    $path = Resolve-Path -Path "$lang\$demo\target\debug\$demo.exe" | Out-Null;

    Measure-Command { $path | Out-Default } | Select-Object @{n="time";e={$_.Minutes,"Minutes",$_.Seconds,"Seconds",$_.TotalMilliseconds,"Milliseconds" -join " "}}
}
elseif ($lang -eq "js") {
    $path = Resolve-Path -Path "$lang\$demo\index.js" | Out-Null;

    Measure-Command { node $path | Out-Default } | Select-Object @{n="time";e={$_.Minutes,"Minutes",$_.Seconds,"Seconds",$_.TotalMilliseconds,"Milliseconds" -join " "}}
}
