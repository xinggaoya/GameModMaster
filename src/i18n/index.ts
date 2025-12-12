import { createI18n } from 'vue-i18n'

export type Locale = 'zh-CN' | 'en-US' | 'es-ES' | 'fr-FR' | 'ja-JP'

const messages = {
  'zh-CN': {
    common: {
      appName: 'GameMod Master',
      back: '返回',
      cancel: '取消',
      clear: '清除',
      search: '搜索',
      refresh: '刷新',
      retry: '重试',
      open: '打开',
      browse: '浏览',
      save: '保存设置',
      startExploring: '开始探索',
      launch: '启动',
      delete: '删除',
      download: '下载',
      downloaded: '已下载',
      openFolder: '打开目录',
      loading: '加载中...',
      errorTitle: '出错了',
      noData: '暂无数据',
    },
    nav: {
      explore: '探索',
      downloads: '收藏',
      settings: '设置',
    },
    home: {
      title: '探索修改器',
      subtitle: '发现上千款游戏修改器，让游戏更有趣',
      stats: {
        available: '可用',
        downloaded: '已下载',
        installed: '已安装',
      },
      searchPlaceholder: '搜索游戏或修改器...',
      results: {
        all: '全部修改器',
        query: '搜索 "{query}"',
        count: '{count} 个结果',
      },
      empty: {
        title: '没有找到结果',
        withQuery: '没有找到 "{query}" 相关的修改器',
        withoutQuery: '暂无可用的修改器',
      },
    },
    downloads: {
      title: '我的收藏',
      subtitle: '管理已下载的游戏修改器',
      stats: {
        downloaded: '已下载',
        installed: '已安装',
      },
      searchPlaceholder: '搜索已下载的修改器...',
      empty: {
        title: '还没有下载任何修改器',
        subtitle: '去探索页面下载游戏修改器吧',
        searchTitle: '没有找到结果',
        searchSubtitle: '没有找到 "{query}" 相关的修改器',
      },
      errors: {
        openFolder: '打开文件夹失败',
      },
    },
    detail: {
      status: {
        downloaded: '已下载',
      },
      progress: {
        downloading: '正在下载... {progress}%',
      },
      actions: {
        launch: '启动修改器',
        delete: '删除',
        download: '下载修改器',
      },
      meta: {
        version: '版本',
        gameVersion: '游戏版本',
        downloads: '下载量',
        lastUpdate: '更新时间',
      },
      description: {
        title: '功能说明',
        empty: '暂无描述',
      },
      messages: {
        fetchFailed: '获取详情失败',
        downloadSuccess: '下载成功',
        downloadFailed: '下载失败',
        launchSuccess: '已启动',
        launchFailed: '启动失败',
        deleteSuccess: '已删除',
        deleteFailed: '删除失败',
      },
      deleteConfirm: {
        title: '确认删除',
        content: '确定要删除 "{name}" 吗？',
        positive: '删除',
        negative: '取消',
      },
    },
    settings: {
      title: '设置',
      subtitle: '自定义您的 GameMod Master 体验',
      sections: {
        download: '下载设置',
        appearance: '外观',
        about: '关于',
        update: '更新',
      },
      fields: {
        downloadPath: {
          label: '下载路径',
          desc: '修改器将保存到此目录',
        },
        autoOpen: {
          label: '下载完成后打开文件夹',
          desc: '下载完成后自动打开所在目录',
        },
        theme: {
          label: '主题',
          desc: '选择应用的外观主题',
        },
        language: {
          label: '界面语言',
          desc: '选择应用界面的显示语言',
        },
      },
      themeOptions: {
        system: '跟随系统',
        light: '浅色模式',
        dark: '深色模式',
      },
      buttons: {
        browse: '浏览',
        open: '打开',
        save: '保存设置',
      },
      about: {
        version: '版本',
        author: '作者',
      },
      messages: {
        saveSuccess: '设置已保存',
        saveFailed: '保存设置失败',
        loadFailed: '加载设置失败',
        selectFolderFailed: '选择文件夹失败',
        openFolderFailed: '打开文件夹失败',
        autoSaveThemeFailed: '自动保存主题失败',
        checkUpdateFailed: '检查更新失败，请稍后再试',
      },
    },
    update: {
      check: '检查更新',
      checking: '检查中...',
      upToDate: '当前已是最新版本',
      foundTitle: '发现新版本',
      downloadingTitle: '正在下载更新',
      currentVersion: '当前版本',
      latestVersion: '最新版本',
      contentTitle: '更新内容',
      start: '立即更新',
      cancel: '取消',
      progress: {
        preparing: '准备下载...',
        failed: '下载失败: {error}',
      },
    },
    progress: {
      status: {
        waiting: '等待中',
        downloading: '下载中',
        processing: '处理中',
        extracting: '解压中',
        completed: '已完成',
        error: '错误',
      },
      cancel: '取消',
    },
    gameCard: {
      viewDetail: '查看详情',
      downloaded: '已下载',
      actions: {
        launch: '启动',
        delete: '删除',
        download: '下载',
        downloaded: '已下载',
      },
      messages: {
        downloadSuccess: '下载成功',
        downloadFailed: '下载失败',
        launchSuccess: '已启动',
        launchFailed: '启动失败',
        deleteSuccess: '已删除',
        deleteFailed: '删除失败',
      },
      deleteConfirm: {
        title: '确认删除',
        content: '确定要删除 "{name}" 吗？',
        positive: '删除',
        negative: '取消',
      },
    },
    logViewer: {
      title: '日志查看器',
      refresh: '刷新',
      export: '导出',
      empty: '没有可用的日志',
      loading: '加载中...',
      loadFailed: '加载日志失败',
      openDirFailed: '打开日志目录失败',
    },
    languages: {
      'zh-CN': '简体中文',
      'en-US': 'English',
      'es-ES': 'Español',
      'fr-FR': 'Français',
      'ja-JP': '日本語',
    },
  },
  'en-US': {
    common: {
      appName: 'GameMod Master',
      back: 'Back',
      cancel: 'Cancel',
      clear: 'Clear',
      search: 'Search',
      refresh: 'Refresh',
      retry: 'Retry',
      open: 'Open',
      browse: 'Browse',
      save: 'Save Settings',
      startExploring: 'Start Exploring',
      launch: 'Launch',
      delete: 'Delete',
      download: 'Download',
      downloaded: 'Downloaded',
      openFolder: 'Open Folder',
      loading: 'Loading...',
      errorTitle: 'Something went wrong',
      noData: 'No data',
    },
    nav: {
      explore: 'Explore',
      downloads: 'Library',
      settings: 'Settings',
    },
    home: {
      title: 'Discover Trainers',
      subtitle: 'Find thousands of game trainers to make gameplay more fun',
      stats: {
        available: 'Available',
        downloaded: 'Downloaded',
        installed: 'Installed',
      },
      searchPlaceholder: 'Search games or trainers...',
      results: {
        all: 'All trainers',
        query: 'Search "{query}"',
        count: '{count} results',
      },
      empty: {
        title: 'No results found',
        withQuery: 'No trainers found for "{query}"',
        withoutQuery: 'No trainers available yet',
      },
    },
    downloads: {
      title: 'My Library',
      subtitle: 'Manage downloaded trainers',
      stats: {
        downloaded: 'Downloaded',
        installed: 'Installed',
      },
      searchPlaceholder: 'Search downloaded trainers...',
      empty: {
        title: 'No trainers downloaded',
        subtitle: 'Go to Explore to download trainers',
        searchTitle: 'No results found',
        searchSubtitle: 'No trainers found for "{query}"',
      },
      errors: {
        openFolder: 'Failed to open folder',
      },
    },
    detail: {
      status: {
        downloaded: 'Downloaded',
      },
      progress: {
        downloading: 'Downloading... {progress}%',
      },
      actions: {
        launch: 'Launch Trainer',
        delete: 'Delete',
        download: 'Download Trainer',
      },
      meta: {
        version: 'Version',
        gameVersion: 'Game Version',
        downloads: 'Downloads',
        lastUpdate: 'Last Updated',
      },
      description: {
        title: 'Features',
        empty: 'No description',
      },
      messages: {
        fetchFailed: 'Failed to load details',
        downloadSuccess: 'Download successful',
        downloadFailed: 'Download failed',
        launchSuccess: 'Launched',
        launchFailed: 'Launch failed',
        deleteSuccess: 'Deleted',
        deleteFailed: 'Delete failed',
      },
      deleteConfirm: {
        title: 'Confirm delete',
        content: 'Are you sure you want to delete "{name}"?',
        positive: 'Delete',
        negative: 'Cancel',
      },
    },
    settings: {
      title: 'Settings',
      subtitle: 'Customize your GameMod Master experience',
      sections: {
        download: 'Download',
        appearance: 'Appearance',
        about: 'About',
        update: 'Updates',
      },
      fields: {
        downloadPath: {
          label: 'Download path',
          desc: 'Trainers will be saved to this folder',
        },
        autoOpen: {
          label: 'Open folder after download',
          desc: 'Automatically open the folder after download completes',
        },
        theme: {
          label: 'Theme',
          desc: 'Choose the app appearance',
        },
        language: {
          label: 'Language',
          desc: 'Choose the interface language',
        },
      },
      themeOptions: {
        system: 'System default',
        light: 'Light mode',
        dark: 'Dark mode',
      },
      buttons: {
        browse: 'Browse',
        open: 'Open',
        save: 'Save Settings',
      },
      about: {
        version: 'Version',
        author: 'Author',
      },
      messages: {
        saveSuccess: 'Settings saved',
        saveFailed: 'Failed to save settings',
        loadFailed: 'Failed to load settings',
        selectFolderFailed: 'Failed to select folder',
        openFolderFailed: 'Failed to open folder',
        autoSaveThemeFailed: 'Failed to auto-save theme',
        checkUpdateFailed: 'Failed to check for updates, please retry later',
      },
    },
    update: {
      check: 'Check for updates',
      checking: 'Checking...',
      upToDate: 'You are up to date',
      foundTitle: 'New version available',
      downloadingTitle: 'Downloading update',
      currentVersion: 'Current version',
      latestVersion: 'Latest version',
      contentTitle: 'Release notes',
      start: 'Update now',
      cancel: 'Cancel',
      progress: {
        preparing: 'Preparing download...',
        failed: 'Download failed: {error}',
      },
    },
    progress: {
      status: {
        waiting: 'Waiting',
        downloading: 'Downloading',
        processing: 'Processing',
        extracting: 'Extracting',
        completed: 'Completed',
        error: 'Error',
      },
      cancel: 'Cancel',
    },
    gameCard: {
      viewDetail: 'View details',
      downloaded: 'Downloaded',
      actions: {
        launch: 'Launch',
        delete: 'Delete',
        download: 'Download',
        downloaded: 'Downloaded',
      },
      messages: {
        downloadSuccess: 'Download successful',
        downloadFailed: 'Download failed',
        launchSuccess: 'Launched',
        launchFailed: 'Launch failed',
        deleteSuccess: 'Deleted',
        deleteFailed: 'Delete failed',
      },
      deleteConfirm: {
        title: 'Confirm delete',
        content: 'Are you sure you want to delete "{name}"?',
        positive: 'Delete',
        negative: 'Cancel',
      },
    },
    logViewer: {
      title: 'Log Viewer',
      refresh: 'Refresh',
      export: 'Export',
      empty: 'No logs available',
      loading: 'Loading...',
      loadFailed: 'Failed to load logs',
      openDirFailed: 'Failed to open log folder',
    },
    languages: {
      'zh-CN': '简体中文',
      'en-US': 'English',
      'es-ES': 'Español',
      'fr-FR': 'Français',
      'ja-JP': '日本語',
    },
  },
  'es-ES': {
    common: {
      appName: 'GameMod Master',
      back: 'Volver',
      cancel: 'Cancelar',
      clear: 'Limpiar',
      search: 'Buscar',
      refresh: 'Actualizar',
      retry: 'Reintentar',
      open: 'Abrir',
      browse: 'Elegir',
      save: 'Guardar ajustes',
      startExploring: 'Empezar a explorar',
      launch: 'Iniciar',
      delete: 'Eliminar',
      download: 'Descargar',
      downloaded: 'Descargado',
      openFolder: 'Abrir carpeta',
      loading: 'Cargando...',
      errorTitle: 'Ha ocurrido un error',
      noData: 'Sin datos',
    },
    nav: {
      explore: 'Explorar',
      downloads: 'Biblioteca',
      settings: 'Ajustes',
    },
    home: {
      title: 'Explorar entrenadores',
      subtitle: 'Descubre miles de entrenadores para hacer tus juegos más divertidos',
      stats: {
        available: 'Disponibles',
        downloaded: 'Descargados',
        installed: 'Instalados',
      },
      searchPlaceholder: 'Buscar juegos o entrenadores...',
      results: {
        all: 'Todos los entrenadores',
        query: 'Buscar "{query}"',
        count: '{count} resultados',
      },
      empty: {
        title: 'No se encontraron resultados',
        withQuery: 'No hay entrenadores para "{query}"',
        withoutQuery: 'Aún no hay entrenadores disponibles',
      },
    },
    downloads: {
      title: 'Mi biblioteca',
      subtitle: 'Gestiona los entrenadores descargados',
      stats: {
        downloaded: 'Descargados',
        installed: 'Instalados',
      },
      searchPlaceholder: 'Buscar entrenadores descargados...',
      empty: {
        title: 'Aún no has descargado entrenadores',
        subtitle: 'Ve a Explorar para descargar entrenadores',
        searchTitle: 'No se encontraron resultados',
        searchSubtitle: 'No hay entrenadores para "{query}"',
      },
      errors: {
        openFolder: 'No se pudo abrir la carpeta',
      },
    },
    detail: {
      status: {
        downloaded: 'Descargado',
      },
      progress: {
        downloading: 'Descargando... {progress}%',
      },
      actions: {
        launch: 'Iniciar entrenador',
        delete: 'Eliminar',
        download: 'Descargar entrenador',
      },
      meta: {
        version: 'Versión',
        gameVersion: 'Versión del juego',
        downloads: 'Descargas',
        lastUpdate: 'Última actualización',
      },
      description: {
        title: 'Funciones',
        empty: 'Sin descripción',
      },
      messages: {
        fetchFailed: 'Error al cargar detalles',
        downloadSuccess: 'Descarga completada',
        downloadFailed: 'Descarga fallida',
        launchSuccess: 'Iniciado',
        launchFailed: 'Error al iniciar',
        deleteSuccess: 'Eliminado',
        deleteFailed: 'Error al eliminar',
      },
      deleteConfirm: {
        title: 'Confirmar eliminación',
        content: '¿Eliminar "{name}"?',
        positive: 'Eliminar',
        negative: 'Cancelar',
      },
    },
    settings: {
      title: 'Ajustes',
      subtitle: 'Personaliza tu experiencia en GameMod Master',
      sections: {
        download: 'Descarga',
        appearance: 'Apariencia',
        about: 'Acerca de',
        update: 'Actualizaciones',
      },
      fields: {
        downloadPath: {
          label: 'Ruta de descarga',
          desc: 'Los entrenadores se guardarán aquí',
        },
        autoOpen: {
          label: 'Abrir carpeta tras descargar',
          desc: 'Abrir automáticamente la carpeta al terminar',
        },
        theme: {
          label: 'Tema',
          desc: 'Elige la apariencia de la app',
        },
        language: {
          label: 'Idioma',
          desc: 'Elige el idioma de la interfaz',
        },
      },
      themeOptions: {
        system: 'Usar sistema',
        light: 'Modo claro',
        dark: 'Modo oscuro',
      },
      buttons: {
        browse: 'Elegir',
        open: 'Abrir',
        save: 'Guardar ajustes',
      },
      about: {
        version: 'Versión',
        author: 'Autor',
      },
      messages: {
        saveSuccess: 'Ajustes guardados',
        saveFailed: 'No se pudieron guardar los ajustes',
        loadFailed: 'No se pudieron cargar los ajustes',
        selectFolderFailed: 'No se pudo seleccionar la carpeta',
        openFolderFailed: 'No se pudo abrir la carpeta',
        autoSaveThemeFailed: 'No se pudo guardar el tema automáticamente',
        checkUpdateFailed: 'No se pudieron buscar actualizaciones, inténtalo más tarde',
      },
    },
    update: {
      check: 'Buscar actualizaciones',
      checking: 'Comprobando...',
      upToDate: 'Ya tienes la última versión',
      foundTitle: 'Nueva versión disponible',
      downloadingTitle: 'Descargando actualización',
      currentVersion: 'Versión actual',
      latestVersion: 'Última versión',
      contentTitle: 'Notas de la versión',
      start: 'Actualizar ahora',
      cancel: 'Cancelar',
      progress: {
        preparing: 'Preparando descarga...',
        failed: 'La descarga falló: {error}',
      },
    },
    progress: {
      status: {
        waiting: 'En espera',
        downloading: 'Descargando',
        processing: 'Procesando',
        extracting: 'Extrayendo',
        completed: 'Completado',
        error: 'Error',
      },
      cancel: 'Cancelar',
    },
    gameCard: {
      viewDetail: 'Ver detalles',
      downloaded: 'Descargado',
      actions: {
        launch: 'Iniciar',
        delete: 'Eliminar',
        download: 'Descargar',
        downloaded: 'Descargado',
      },
      messages: {
        downloadSuccess: 'Descarga completada',
        downloadFailed: 'Descarga fallida',
        launchSuccess: 'Iniciado',
        launchFailed: 'Error al iniciar',
        deleteSuccess: 'Eliminado',
        deleteFailed: 'Error al eliminar',
      },
      deleteConfirm: {
        title: 'Confirmar eliminación',
        content: '¿Eliminar "{name}"?',
        positive: 'Eliminar',
        negative: 'Cancelar',
      },
    },
    logViewer: {
      title: 'Visor de registros',
      refresh: 'Actualizar',
      export: 'Exportar',
      empty: 'No hay registros disponibles',
      loading: 'Cargando...',
      loadFailed: 'No se pudieron cargar los registros',
      openDirFailed: 'No se pudo abrir la carpeta de registros',
    },
    languages: {
      'zh-CN': '简体中文',
      'en-US': 'English',
      'es-ES': 'Español',
      'fr-FR': 'Français',
      'ja-JP': '日本語',
    },
  },
  'fr-FR': {
    common: {
      appName: 'GameMod Master',
      back: 'Retour',
      cancel: 'Annuler',
      clear: 'Effacer',
      search: 'Rechercher',
      refresh: 'Rafraîchir',
      retry: 'Réessayer',
      open: 'Ouvrir',
      browse: 'Parcourir',
      save: 'Enregistrer',
      startExploring: 'Commencer à explorer',
      launch: 'Lancer',
      delete: 'Supprimer',
      download: 'Télécharger',
      downloaded: 'Téléchargé',
      openFolder: 'Ouvrir le dossier',
      loading: 'Chargement...',
      errorTitle: 'Une erreur est survenue',
      noData: 'Aucune donnée',
    },
    nav: {
      explore: 'Découvrir',
      downloads: 'Bibliothèque',
      settings: 'Paramètres',
    },
    home: {
      title: 'Découvrir des trainers',
      subtitle: 'Trouvez des milliers de trainers pour rendre vos jeux plus amusants',
      stats: {
        available: 'Disponibles',
        downloaded: 'Téléchargés',
        installed: 'Installés',
      },
      searchPlaceholder: 'Rechercher des jeux ou des trainers...',
      results: {
        all: 'Tous les trainers',
        query: 'Recherche "{query}"',
        count: '{count} résultats',
      },
      empty: {
        title: 'Aucun résultat',
        withQuery: 'Aucun trainer trouvé pour "{query}"',
        withoutQuery: 'Aucun trainer disponible pour le moment',
      },
    },
    downloads: {
      title: 'Ma bibliothèque',
      subtitle: 'Gérer les trainers téléchargés',
      stats: {
        downloaded: 'Téléchargés',
        installed: 'Installés',
      },
      searchPlaceholder: 'Rechercher dans les téléchargements...',
      empty: {
        title: 'Aucun trainer téléchargé',
        subtitle: 'Allez dans Découvrir pour en télécharger',
        searchTitle: 'Aucun résultat',
        searchSubtitle: 'Aucun trainer trouvé pour "{query}"',
      },
      errors: {
        openFolder: 'Impossible d’ouvrir le dossier',
      },
    },
    detail: {
      status: {
        downloaded: 'Téléchargé',
      },
      progress: {
        downloading: 'Téléchargement... {progress}%',
      },
      actions: {
        launch: 'Lancer le trainer',
        delete: 'Supprimer',
        download: 'Télécharger le trainer',
      },
      meta: {
        version: 'Version',
        gameVersion: 'Version du jeu',
        downloads: 'Téléchargements',
        lastUpdate: 'Dernière mise à jour',
      },
      description: {
        title: 'Fonctionnalités',
        empty: 'Pas de description',
      },
      messages: {
        fetchFailed: 'Échec du chargement des détails',
        downloadSuccess: 'Téléchargement réussi',
        downloadFailed: 'Échec du téléchargement',
        launchSuccess: 'Lancement réussi',
        launchFailed: 'Échec du lancement',
        deleteSuccess: 'Supprimé',
        deleteFailed: 'Échec de la suppression',
      },
      deleteConfirm: {
        title: 'Confirmer la suppression',
        content: 'Supprimer « {name} » ?',
        positive: 'Supprimer',
        negative: 'Annuler',
      },
    },
    settings: {
      title: 'Paramètres',
      subtitle: 'Personnalisez votre expérience GameMod Master',
      sections: {
        download: 'Téléchargement',
        appearance: 'Apparence',
        about: 'À propos',
        update: 'Mises à jour',
      },
      fields: {
        downloadPath: {
          label: 'Dossier de téléchargement',
          desc: 'Les trainers seront enregistrés ici',
        },
        autoOpen: {
          label: 'Ouvrir le dossier après téléchargement',
          desc: 'Ouvrir automatiquement une fois terminé',
        },
        theme: {
          label: 'Thème',
          desc: 'Choisissez l’apparence de l’application',
        },
        language: {
          label: 'Langue',
          desc: 'Choisissez la langue de l’interface',
        },
      },
      themeOptions: {
        system: 'Suivre le système',
        light: 'Mode clair',
        dark: 'Mode sombre',
      },
      buttons: {
        browse: 'Parcourir',
        open: 'Ouvrir',
        save: 'Enregistrer',
      },
      about: {
        version: 'Version',
        author: 'Auteur',
      },
      messages: {
        saveSuccess: 'Paramètres enregistrés',
        saveFailed: 'Échec de l’enregistrement',
        loadFailed: 'Échec du chargement des paramètres',
        selectFolderFailed: 'Impossible de choisir le dossier',
        openFolderFailed: 'Impossible d’ouvrir le dossier',
        autoSaveThemeFailed: 'Impossible d’enregistrer le thème automatiquement',
        checkUpdateFailed: 'Impossible de vérifier les mises à jour, veuillez réessayer',
      },
    },
    update: {
      check: 'Vérifier les mises à jour',
      checking: 'Vérification...',
      upToDate: 'Vous êtes à jour',
      foundTitle: 'Nouvelle version disponible',
      downloadingTitle: 'Téléchargement de la mise à jour',
      currentVersion: 'Version actuelle',
      latestVersion: 'Dernière version',
      contentTitle: 'Notes de version',
      start: 'Mettre à jour',
      cancel: 'Annuler',
      progress: {
        preparing: 'Préparation du téléchargement...',
        failed: 'Échec du téléchargement : {error}',
      },
    },
    progress: {
      status: {
        waiting: 'En attente',
        downloading: 'Téléchargement',
        processing: 'Traitement',
        extracting: 'Extraction',
        completed: 'Terminé',
        error: 'Erreur',
      },
      cancel: 'Annuler',
    },
    gameCard: {
      viewDetail: 'Voir les détails',
      downloaded: 'Téléchargé',
      actions: {
        launch: 'Lancer',
        delete: 'Supprimer',
        download: 'Télécharger',
        downloaded: 'Téléchargé',
      },
      messages: {
        downloadSuccess: 'Téléchargement réussi',
        downloadFailed: 'Échec du téléchargement',
        launchSuccess: 'Lancement réussi',
        launchFailed: 'Échec du lancement',
        deleteSuccess: 'Supprimé',
        deleteFailed: 'Échec de la suppression',
      },
      deleteConfirm: {
        title: 'Confirmer la suppression',
        content: 'Supprimer « {name} » ?',
        positive: 'Supprimer',
        negative: 'Annuler',
      },
    },
    logViewer: {
      title: 'Visionneuse de logs',
      refresh: 'Rafraîchir',
      export: 'Exporter',
      empty: 'Aucun log disponible',
      loading: 'Chargement...',
      loadFailed: 'Impossible de charger les logs',
      openDirFailed: 'Impossible d’ouvrir le dossier des logs',
    },
    languages: {
      'zh-CN': '简体中文',
      'en-US': 'English',
      'es-ES': 'Español',
      'fr-FR': 'Français',
      'ja-JP': '日本語',
    },
  },
  'ja-JP': {
    common: {
      appName: 'GameMod Master',
      back: '戻る',
      cancel: 'キャンセル',
      clear: 'クリア',
      search: '検索',
      refresh: '更新',
      retry: '再試行',
      open: '開く',
      browse: '参照',
      save: '設定を保存',
      startExploring: '探索を始める',
      launch: '起動',
      delete: '削除',
      download: 'ダウンロード',
      downloaded: 'ダウンロード済み',
      openFolder: 'フォルダーを開く',
      loading: '読み込み中...',
      errorTitle: 'エラーが発生しました',
      noData: 'データがありません',
    },
    nav: {
      explore: '探索',
      downloads: 'ライブラリ',
      settings: '設定',
    },
    home: {
      title: 'トレーナーを探索',
      subtitle: '何千ものゲームトレーナーを見つけて、プレイをもっと楽しく',
      stats: {
        available: '利用可能',
        downloaded: 'ダウンロード済み',
        installed: 'インストール済み',
      },
      searchPlaceholder: 'ゲームまたはトレーナーを検索...',
      results: {
        all: 'すべてのトレーナー',
        query: '"{query}" を検索',
        count: '{count} 件',
      },
      empty: {
        title: '結果が見つかりません',
        withQuery: '"{query}" に一致するトレーナーはありません',
        withoutQuery: '利用できるトレーナーがまだありません',
      },
    },
    downloads: {
      title: 'マイライブラリ',
      subtitle: 'ダウンロード済みトレーナーを管理',
      stats: {
        downloaded: 'ダウンロード済み',
        installed: 'インストール済み',
      },
      searchPlaceholder: 'ダウンロード済みのトレーナーを検索...',
      empty: {
        title: 'まだトレーナーをダウンロードしていません',
        subtitle: '「探索」からトレーナーをダウンロードしましょう',
        searchTitle: '結果が見つかりません',
        searchSubtitle: '"{query}" に一致するトレーナーはありません',
      },
      errors: {
        openFolder: 'フォルダーを開けませんでした',
      },
    },
    detail: {
      status: {
        downloaded: 'ダウンロード済み',
      },
      progress: {
        downloading: 'ダウンロード中... {progress}%',
      },
      actions: {
        launch: 'トレーナーを起動',
        delete: '削除',
        download: 'トレーナーをダウンロード',
      },
      meta: {
        version: 'バージョン',
        gameVersion: 'ゲームバージョン',
        downloads: 'ダウンロード数',
        lastUpdate: '最終更新',
      },
      description: {
        title: '機能',
        empty: '説明はありません',
      },
      messages: {
        fetchFailed: '詳細の取得に失敗しました',
        downloadSuccess: 'ダウンロード成功',
        downloadFailed: 'ダウンロード失敗',
        launchSuccess: '起動しました',
        launchFailed: '起動に失敗しました',
        deleteSuccess: '削除しました',
        deleteFailed: '削除に失敗しました',
      },
      deleteConfirm: {
        title: '削除の確認',
        content: '"{name}" を削除しますか？',
        positive: '削除',
        negative: 'キャンセル',
      },
    },
    settings: {
      title: '設定',
      subtitle: 'GameMod Master の体験をカスタマイズ',
      sections: {
        download: 'ダウンロード',
        appearance: '外観',
        about: '概要',
        update: 'アップデート',
      },
      fields: {
        downloadPath: {
          label: 'ダウンロード先',
          desc: 'トレーナーはこのフォルダーに保存されます',
        },
        autoOpen: {
          label: 'ダウンロード後にフォルダーを開く',
          desc: '完了後に自動でフォルダーを開きます',
        },
        theme: {
          label: 'テーマ',
          desc: 'アプリの外観を選択',
        },
        language: {
          label: '表示言語',
          desc: 'インターフェースの言語を選択',
        },
      },
      themeOptions: {
        system: 'システムに合わせる',
        light: 'ライトモード',
        dark: 'ダークモード',
      },
      buttons: {
        browse: '参照',
        open: '開く',
        save: '設定を保存',
      },
      about: {
        version: 'バージョン',
        author: '作者',
      },
      messages: {
        saveSuccess: '設定を保存しました',
        saveFailed: '設定の保存に失敗しました',
        loadFailed: '設定の読み込みに失敗しました',
        selectFolderFailed: 'フォルダーを選択できませんでした',
        openFolderFailed: 'フォルダーを開けませんでした',
        autoSaveThemeFailed: 'テーマの自動保存に失敗しました',
        checkUpdateFailed: 'アップデートの確認に失敗しました。時間をおいて再度お試しください',
      },
    },
    update: {
      check: 'アップデートを確認',
      checking: '確認中...',
      upToDate: '最新バージョンです',
      foundTitle: '新しいバージョンがあります',
      downloadingTitle: 'アップデートをダウンロード中',
      currentVersion: '現在のバージョン',
      latestVersion: '最新バージョン',
      contentTitle: '更新内容',
      start: '今すぐ更新',
      cancel: 'キャンセル',
      progress: {
        preparing: 'ダウンロードを準備中...',
        failed: 'ダウンロードに失敗しました: {error}',
      },
    },
    progress: {
      status: {
        waiting: '待機中',
        downloading: 'ダウンロード中',
        processing: '処理中',
        extracting: '展開中',
        completed: '完了',
        error: 'エラー',
      },
      cancel: 'キャンセル',
    },
    gameCard: {
      viewDetail: '詳細を見る',
      downloaded: 'ダウンロード済み',
      actions: {
        launch: '起動',
        delete: '削除',
        download: 'ダウンロード',
        downloaded: 'ダウンロード済み',
      },
      messages: {
        downloadSuccess: 'ダウンロード成功',
        downloadFailed: 'ダウンロード失敗',
        launchSuccess: '起動しました',
        launchFailed: '起動に失敗しました',
        deleteSuccess: '削除しました',
        deleteFailed: '削除に失敗しました',
      },
      deleteConfirm: {
        title: '削除の確認',
        content: '"{name}" を削除しますか？',
        positive: '削除',
        negative: 'キャンセル',
      },
    },
    logViewer: {
      title: 'ログビューア',
      refresh: '更新',
      export: 'エクスポート',
      empty: '利用できるログがありません',
      loading: '読み込み中...',
      loadFailed: 'ログの読み込みに失敗しました',
      openDirFailed: 'ログフォルダーを開けませんでした',
    },
    languages: {
      'zh-CN': '简体中文',
      'en-US': 'English',
      'es-ES': 'Español',
      'fr-FR': 'Français',
      'ja-JP': '日本語',
    },
  },
}

export const supportedLanguages: Array<{ label: string; value: Locale }> = [
  { label: messages['zh-CN'].languages['zh-CN'], value: 'zh-CN' },
  { label: messages['en-US'].languages['en-US'], value: 'en-US' },
  { label: messages['es-ES'].languages['es-ES'], value: 'es-ES' },
  { label: messages['fr-FR'].languages['fr-FR'], value: 'fr-FR' },
  { label: messages['ja-JP'].languages['ja-JP'], value: 'ja-JP' },
]

const detectLocale = (): Locale => {
  const saved = localStorage.getItem('language') as Locale | null
  if (saved && messages[saved]) return saved

  const browser = navigator.language
  const match = Object.keys(messages).find((key) => browser.startsWith(key.split('-')[0]))
  return (match as Locale) || 'en-US'
}

const i18n = createI18n({
  legacy: false,
  locale: detectLocale(),
  fallbackLocale: 'en-US',
  messages,
})

document.documentElement.lang = i18n.global.locale.value

export const setLocale = (locale: Locale) => {
  if (!messages[locale]) return
  i18n.global.locale.value = locale
  localStorage.setItem('language', locale)
  document.documentElement.lang = locale
}

export default i18n
