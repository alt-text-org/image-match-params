library('tidyverse')
library('ggridges')
dists = rbind(
  read_csv('grid.orig.csv') %>% mutate(ver="orig")
  read_csv('grid.before.csv') %>% mutate(ver="before")
  read_csv('grid.after.csv') %>% mutate(ver="after")
)
sq = ggplot(dists %>% arrange(left, right, left_path, right_path), aes(
   x=paste(left, left_path),
   y=paste(right, right_path),
   fill=similarity
 )) +
  geom_raster() +
  theme(axis.text.x=element_text(angle=90))

reldists = dists %>%
  filter(left_path == "pics/original") %>%
  mutate(
    is_same_image = left == right,
    rel = ordered(
      ifelse(is_same_image,
        sapply(strsplit(right_path, "/", fixed=T), "[[", 2),
        "other image"
      ),
      c("original", "grown", "shrunk", "cropped", "other image")
    )
  )

p = ggplot(reldists %>% filter(version="orig"), aes(x=similarity, y=left, color=rel, size=(left==right))) +
  geom_density_ridges(data = function(x) { x %>% filter(!is_same_image) }, color="#444", fill="#ccc") +
  geom_point(data = function(x) { x %>% filter(is_same_image) }) +
  scale_size_manual(values=c(0.5,2), guide=F) +
  scale_color_brewer(type="qual", palette="Set1") +
  lims(x=c(-1.00001,1.00001)) +
  labs(y="", color="Relationship")

compare = reldists %>%
  group_by(left) %>%
  mutate(max_other = max(ifelse(is_same_image, similarity, 0))) %>%
  filter(is_same_image) %>%
  summarize(sumdiff = ifelse(
    min(similarity) > max_other,
    sum(similarity - max_other),
    sum(ifelse(similarity < max_other, similarity - max_other, 0))
  )

b = ggplot(compare, aes(x=sumdiff, y=version)) + geom_boxplot()
