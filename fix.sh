path=$1
files=$(ls $path/assets)
for filename in $files
do  
    case "$filename" in
	*.js ) 
    echo "<script type=\"module\" crossorigin src=\"/assets/$filename\"></script>" >> $path/index.html;;
	*.css ) 
    echo "<link rel=\"stylesheet\" crossorigin href=\"/assets/$filename\">" >> $path/index.html;;
    esac
done