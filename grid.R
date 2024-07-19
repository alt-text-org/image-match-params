library('tidyverse')
dists = read_csv('grid.csv')
sq = ggplot(dists %>% arrange(left, right, left_path, right_path), aes(
   x=paste(left, left_path),
   y=paste(right, right_path),
   fill=distance
 )) +
  geom_raster() +
  theme(axis.text.x=element_text(angle=90))

reldists = dists %>%
  filter(left_path == "pics/original") %>%
  mutate(rel = ifelse(left == right, sapply(strsplit(right_path, "/", fixed=T), "[[", 2), "other"))

p = ggplot(reldists, aes(x=distance, y=left, color=rel, size=(left==right))) +
  geom_point() +
  scale_size_manual(values=c(0.5,2), guide=F) +
  labs(y="", color="Relationship")
