#!/usr/bin/env bash
set -x
set -eo pipefail

mkdir -p $GIT_SHA

echo "PDF equality checking has failed." >pr_comment.txt
echo >>pr_comment
echo >>pr_comment

for FILE in $(find tests/files -name '*.pdf' -not -name '*.new.pdf'); do
    echo "Converting file $FILE"
    magick convert \
        -density 300 \
        -define png:color-type=6 \
        -resize 25% \
        $FILE \
        -alpha remove \
        -background white \
        -append \
        ${FILE}.png
    ls -hl ${FILE}.png

    NEW=$(echo $FILE | sed -e 's/.pdf/.new.pdf/')

    echo "Converting file $NEW"
    magick convert \
        -density 300 \
        -define png:color-type=6 \
        -resize 25% \
        $NEW \
        -alpha remove \
        -background white \
        -append \
        ${NEW}.png
    ls -hl ${NEW}.png

    echo "Comparing ${FILE}.png against ${NEW}.png"
    magick compare ${FILE}.png ${NEW}.png ${FILE}.diff.png || EXIT_CODE=$?
    ls -hl ${FILE}.diff.png

    echo "Montaging ${FILE}.png ${NEW}.png ${FILE}.diff.png"
    magick montage ${FILE}.png ${NEW}.png ${FILE}.diff.png -geometry +10+10 ${FILE}.montage.png
    ls -hl ${FILE}.montage.png

    if [[ "$EXIT_CODE" -eq 1 ]]; then
        echo "Moving ${FILE}.montage.png to be uploaded"
        cp "${FILE}.montage.png" "${GIT_SHA}/${FILE}.montage.png"
        echo "<img src=\"${IMAGES_BASE_URL}/pdf-compare/${GIT_SHA}/${FILE}.montage.png\" />" >>pr_comment.txt
    fi
done
