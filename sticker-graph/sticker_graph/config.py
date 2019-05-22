from os import path


class DefaultConfig:
    crf = False
    glu = True
    gru = False
    hidden_size = 50
    keep_prob = 0.85
    keep_prob_input = 0.80
    kernel_size = 3
    n_levels = 7
    rnn_layers = 2
    top_k = 3


def path_relative_to_conf(conf_path, file_path):
    if path.isabs(file_path):
        return path

    return "%s/%s" % (path.dirname(path.abspath(conf_path)), file_path)